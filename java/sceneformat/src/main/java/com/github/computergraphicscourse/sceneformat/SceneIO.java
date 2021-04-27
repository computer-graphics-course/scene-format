package com.github.computergraphicscourse.sceneformat;

import static com.github.computergraphicscourse.sceneformat.SceneFormat.*;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

public interface SceneIO {

  Scene decode(byte[] data) throws IOException;
  Scene decode(String data) throws IOException;

  Scene read(InputStream inputStream) throws IOException;
  Scene read(File file) throws IOException;
  Scene read(String inputPath) throws IOException;

  byte[] encode(Scene scene) throws IOException;
  String encodeAsJson(Scene scene) throws IOException;

  void save(Scene scene, OutputStream outputStream) throws IOException;
  void save(Scene scene, File file) throws IOException;
  void save(Scene scene, String outputPath) throws IOException;

  void saveAsJson(Scene scene, OutputStream outputStream) throws IOException;
  void saveAsJson(Scene scene, File file) throws IOException;
  void saveAsJson(Scene scene, String outputPath) throws IOException;
}
