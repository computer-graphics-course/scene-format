package com.github.computergraphicscourse.sceneformat;

import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.util.JsonFormat;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import org.apache.commons.io.IOUtils;

public class SceneFormatIO implements SceneIO {

  @Override
  public SceneFormat.Scene decode(byte[] data) throws IOException {
    try {
      var builder = SceneFormat.Scene.newBuilder();
      JsonFormat.parser().merge(new String(data, StandardCharsets.UTF_8), builder);
      return builder.build();
    } catch (InvalidProtocolBufferException e) {
      return SceneFormat.Scene.parseFrom(data);
    }
  }

  @Override
  public SceneFormat.Scene decode(String data) throws IOException {
    return decode(data.getBytes(StandardCharsets.UTF_8));
  }

  @Override
  public SceneFormat.Scene read(InputStream inputStream) throws IOException {
    return decode(IOUtils.toByteArray(inputStream));
  }

  @Override
  public SceneFormat.Scene read(File file) throws IOException {
    return read(new FileInputStream(file));
  }

  @Override
  public SceneFormat.Scene read(String inputPath) throws IOException {
    return read(new File(inputPath));
  }

  @Override
  public byte[] encode(SceneFormat.Scene scene) throws IOException {
    ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
    scene.writeTo(outputStream);
    return outputStream.toByteArray();
  }

  @Override
  public String encodeAsJson(SceneFormat.Scene scene) throws IOException {
    return JsonFormat.printer().print(scene);
  }

  @Override
  public void save(SceneFormat.Scene scene, OutputStream outputStream) throws IOException {
    scene.writeTo(outputStream);
  }

  @Override
  public void save(SceneFormat.Scene scene, File file) throws IOException {
    save(scene, new FileOutputStream(file));
  }

  @Override
  public void save(SceneFormat.Scene scene, String outputPath) throws IOException {
    save(scene, new File(outputPath));
  }

  @Override
  public void saveAsJson(SceneFormat.Scene scene, OutputStream outputStream) throws IOException {
    outputStream.write(encodeAsJson(scene).getBytes(StandardCharsets.UTF_8));
  }

  @Override
  public void saveAsJson(SceneFormat.Scene scene, File file) throws IOException {
    saveAsJson(scene, new FileOutputStream(file));
  }

  @Override
  public void saveAsJson(SceneFormat.Scene scene, String outputPath) throws IOException {
    saveAsJson(scene, new File(outputPath));
  }
}
