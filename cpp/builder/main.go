package main

import (
	"archive/zip"
	"context"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"

	"cloud.google.com/go/storage"
)

func main() {
	fmt.Println("Build server started")

	httpPrefix := httpPrefix()
	http.HandleFunc(httpPrefix+"/cpp", cpp)
	http.HandleFunc(httpPrefix+"/", notFound)

	port, err := port()
	if err != nil {
		fmt.Println("Failed to get port envvar:", err)
		return
	}

	http.ListenAndServe(":"+strconv.Itoa(port), nil)
}

func httpPrefix() string {
	return os.Getenv("HTTP_PREFIX")
}

func port() (int, error) {
	value := os.Getenv("PORT")
	if len(value) == 0 {
		return 8080, nil
	}

	return strconv.Atoi(value)
}

func notFound(w http.ResponseWriter, req *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	w.Write([]byte("Not found: " + req.URL.Path))
	fmt.Println("Replying not found for request to (from root):", req.URL.Path, "using http prefix:", httpPrefix())
}

func cpp(w http.ResponseWriter, req *http.Request) {
	if !checkRequest(w, req) {
		return
	}

	branchName := req.URL.Query().Get("branch")
	version := req.URL.Query().Get("version")
	repoPath := "/tmp/scene-format-cpp"
	archiveName := "cpp" + version + ".zip"
	archivePath := "/tmp/" + archiveName

	fmt.Println("Cloning branch", branchName)

	err := cloneRepo(branchName, repoPath)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("Internal server error."))
		fmt.Println("Failed to clone repo:", err)
		return
	}
	defer os.RemoveAll(repoPath)

	err = runProtoc(repoPath)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("Internal server error."))
		fmt.Println("Failed to run protoc in cloned repo:", err)
		return
	}

	err = compress(repoPath+"/cpp/sceneformat", archivePath)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("Internal server error."))
		fmt.Println("Failed to compress source files:", err)
		return
	}

	err = uploadToBucket(archivePath, "turbocow", "sceneformat/releases/"+archiveName)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("Internal server error."))
		fmt.Println("Failed to upload version archive for cpp:", err)
		return
	}

	err = uploadToBucket(archivePath, "turbocow", "sceneformat/releases/cpp_latest.zip")
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("Internal server error."))
		fmt.Println("Failed to upload latest archive for cpp:", err)
		return
	}

	w.Write([]byte("Ok."))
}

func checkRequest(w http.ResponseWriter, req *http.Request) bool {
	if req.URL.Path != httpPrefix()+"/cpp" {
		w.WriteHeader(http.StatusNotFound)
		w.Write([]byte("Not found: " + req.URL.Path))
		fmt.Println("Replying not found for request to:", req.URL.Path)
		return false
	}

	if req.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		w.Write([]byte("Only POST is allowed at this endpoint."))
		return false
	}

	if req.URL.Query().Get("branch") == "" {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte("Branch name not set."))
		return false
	}

	if req.URL.Query().Get("version") == "" {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte("Version not set."))
		return false
	}

	return true
}

func cloneRepo(branchName string, cloneTo string) error {
	cmd := exec.Command(
		"git", "clone",
		"--single-branch",
		"--branch", branchName,
		"--depth", "1",
		"https://github.com/computer-graphics-course/scene-format.git",
		cloneTo,
	)
	return cmd.Run()
}

func runProtoc(pathToRepo string) error {
	cmd := exec.Command(
		"protoc",
		"-I="+pathToRepo+"/proto",
		pathToRepo+"/proto/scene.proto",
		"--cpp_out="+pathToRepo+"/cpp/sceneformat",
	)

	output, err := cmd.CombinedOutput()
	if err != nil {
		fmt.Println("protoc output:", string(output))
		return err
	}

	return nil
}

func compress(pathToDirectory string, pathToArchive string) error {
	zipFile, err := os.Create(pathToArchive)
	if err != nil {
		return err
	}
	defer zipFile.Close()

	zipWriter := zip.NewWriter(zipFile)
	defer zipWriter.Close()

	files, err := ioutil.ReadDir(pathToDirectory)
	if err != nil {
		return err
	}

	for _, file := range files {
		if !(strings.HasSuffix(file.Name(), ".cpp") || strings.HasSuffix(file.Name(), ".cc") || strings.HasSuffix(file.Name(), ".h")) {
			continue
		}

		fileToZip, err := os.Open(filepath.Join(pathToDirectory, file.Name()))
		if err != nil {
			return err
		}

		info, err := fileToZip.Stat()
		if err != nil {
			return err
		}

		header, err := zip.FileInfoHeader(info)
		if err != nil {
			return err
		}

		header.Name = file.Name()
		header.Method = zip.Deflate

		writer, err := zipWriter.CreateHeader(header)
		if err != nil {
			return err
		}

		_, err = io.Copy(writer, fileToZip)
		if err != nil {
			return err
		}
	}

	return nil
}

func uploadToBucket(archivePath string, bucketName string, bucketPath string) error {
	objectBytes, err := ioutil.ReadFile(archivePath)
	if err != nil {
		return err
	}

	ctx := context.Background()
	client, err := storage.NewClient(ctx)
	if err != nil {
		return err
	}

	bucket := client.Bucket(bucketName)
	obj := bucket.Object(bucketPath)

	writer := obj.NewWriter(ctx)
	_, err = writer.Write(objectBytes)
	if err != nil {
		return err
	}
	defer writer.Close()

	return nil
}
