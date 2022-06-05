docker run --rm \
  -v $(pwd)/docs:/out \
  -v $(pwd)/proto:/protos \
  pseudomuto/protoc-gen-doc --doc_opt=/out/html.tmpl,index.html