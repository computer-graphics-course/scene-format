#include <iostream>
#include "sceneio.h"
#include <google/protobuf/util/json_util.h>

using namespace scene_format;

int main()
{
    scene_format::Scene scene;
    scene.set_version(1);

    auto scene_object = scene.add_scene_objects();
    scene_object->set_id(0);

    scene_object->mutable_transform()->mutable_position()->set_x(1.0);
    scene_object->mutable_transform()->mutable_position()->set_y(1.0);
    scene_object->mutable_transform()->mutable_position()->set_z(1.0);

    scene_object->mutable_material()->mutable_solid();

    *scene_object->mutable_meshed_object()->mutable_reference() = "cow.obj";

    auto camera = scene.add_cameras();
    camera->set_id(0);
    
    camera->mutable_transform()->mutable_position()->set_x(1.01);
    camera->mutable_transform()->mutable_position()->set_y(2.76);
    camera->mutable_transform()->mutable_position()->set_z(3);

    camera->mutable_perspective()->set_fov(60);

    scene_format::SceneIO* io = new scene_format::SceneFormatIO();
    io->save(scene, "example_binary.cowscene");
    io->save_as_json(scene, "example_json.cowscene");

    auto read_result_binary = io->read("example_binary.cowscene");
    std::cout << "Camera X is " << read_result_binary.cameras(0).transform().position().x() << " when reading binary" << std::endl;

    auto read_result_json = io->read("example_json.cowscene");
    std::cout << "Camera X is " << read_result_json.cameras(0).transform().position().x() << " when reading json" << std::endl;
}
