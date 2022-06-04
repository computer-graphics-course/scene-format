using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace SceneFormat
{
    static class ScenePostProcessing
    {
        private static double TOLERANCE = 0.00001;

        public static Scene PostProcessAndValidate(Scene scene)
        {
            return Validate(PostProcess(scene));
        }
        
        public static Scene PostProcess(Scene scene)
        {
            if (scene.Version == 0)
            {
                scene.Version = 1;
            }

            if (scene.RenderOptions != null)
            {
                PostProcessRenderOptions(scene.RenderOptions);
            }
            
            if (scene.SceneObjects != null)
            {
                PostProcessSceneObjects(scene.SceneObjects.ToList());
            }

            if (scene.Lights != null)
            {
                PostProcessLights(scene.Lights.ToList());
            }

            if (scene.Cameras != null)
            {
                PostProcessCameras(scene.Cameras.ToList());
            }

            if (scene.Materials != null)
            {
                PostProcessMaterials(scene.Materials.ToList());
            }
            
            return scene;
        }

        private static void PostProcessSceneObjects(List<SceneObject> sceneObjects)
        {
            foreach (var sceneObject in sceneObjects)
            {
                PostProcessSceneObject(sceneObject);
            }
        }
        
        private static void PostProcessSceneObject(SceneObject sceneObject)
        {
            if (sceneObject.Transform == null)
            {
                sceneObject.Transform = new Transform { };
            }
            PostProcessTransform(sceneObject.Transform);
            if (sceneObject.Material != null)
            {
                PostProcessMaterial(sceneObject.Material);
            }
            
            if (sceneObject.Cube != null)
            {
                PostProcessCube(sceneObject.Cube);
            }

            if (sceneObject.Disk != null)
            {
                PostProcessDisk(sceneObject.Disk);
            }

            if (sceneObject.Sphere != null)
            {
                PostProcessSphere(sceneObject.Sphere);
            }
        }
        
        private static void PostProcessCube(Cube cube)
        {
            cube.Size ??= new Vector3
            {
                X = 1,
                Y = 1,
                Z = 1,
            };
        }

        private static void PostProcessDisk(Disk disk)
        {
            if (disk.Radius == 0)
            {
                disk.Radius = 1;
            }
        }

        private static void PostProcessSphere(Sphere sphere)
        {
            if (sphere.Radius == 0)
            {
                sphere.Radius = 1;
            }
        }

        private static void PostProcessLights(List<Light> lights)
        {
            foreach (var light in lights)
            {
                PostProcessLight(light);
            }
        }
        
        private static void PostProcessLight(Light light)
        {
            if (light.Transform == null)
            {
                light.Transform = new Transform { };
            }
            PostProcessTransform(light.Transform);
            
            light.Color ??= new Color
            {
                R = 1,
                G = 1,
                B = 1,
            };

            if (light.Sphere != null)
            {
                PostProcessLightSphere(light.Sphere);
            }
        }

        private static void PostProcessLightSphere(LightSphere sphere)
        {
            if (sphere.Radius == 0)
            {
                sphere.Radius = 1;
            }
        }

        private static void PostProcessCameras(List<Camera> cameras)
        {
            foreach (var camera in cameras)
            {
                PostProcessCamera(camera);
            }
        }
        
        private static void PostProcessCamera(Camera camera)
        {
            if (camera.Transform == null)
            {
                camera.Transform = new Transform { };
            }
            PostProcessTransform(camera.Transform);
            
            if (camera.Perspective != null)
            {
                PostProcessCamera(camera.Perspective);
            }
            if (camera.Realistic != null)
            {
                PostProcessCamera(camera.Realistic);
            }
        }
        
        private static void PostProcessCamera(PerspectiveCamera camera)
        {
            if (camera.Fov == 0)
            {
                camera.Fov = 60;
            }
        }

        private static void PostProcessCamera(RealisticCamera camera)
        {
            if (camera.Fov == 0)
            {
                camera.Fov = 60;
            }
            if (camera.Focus == 0)
            {
                camera.Focus = 1;
            }
            if (camera.Radius == 0)
            {
                camera.Focus = 0.01;
            }
        }
        
        private static void PostProcessRenderOptions(RenderOptions renderOptions)
        {
            if (renderOptions.CameraId == 0)
            {
                renderOptions.CameraId = 1;
            }

            if (renderOptions.Width == 0)
            {
                renderOptions.Width = 640;
            }

            if (renderOptions.Height == 0)
            {
                renderOptions.Height = 320;
            }
            
            if (renderOptions.RayDepth == 0)
            {
                renderOptions.RayDepth = 3;
            }
            
            if (renderOptions.RaysPerPixelDimension == 0)
            {
                renderOptions.RaysPerPixelDimension = 1;
            }
        }

        private static void PostProcessMaterials(List<Material> materials)
        {
            foreach (var material in materials)
            {
                PostProcessMaterial(material);
            }
        }
        
        private static void PostProcessMaterial(Material material)
        {
            if (material.LambertReflection != null)
            {
                PostProcessMaterial(material.LambertReflection);
            }
            if (material.SpecularReflection != null)
            {
                PostProcessMaterial(material.SpecularReflection);
            }
            if (material.SpecularTransmission != null)
            {
                PostProcessMaterial(material.SpecularTransmission);
            }
            if (material.Fresnel != null)
            {
                PostProcessMaterial(material.Fresnel);
            }
            if (material.OrenNayar != null)
            {
                PostProcessMaterial(material.OrenNayar);
            }
            if (material.MicrofacetReflection != null)
            {
                PostProcessMaterial(material.MicrofacetReflection);
            }
            if (material.Metal != null)
            {
                PostProcessMaterial(material.Metal);
            }
            if (material.Plastic != null)
            {
                PostProcessMaterial(material.Plastic);
            }
            if (material.Blend != null)
            {
                PostProcessMaterial(material.Blend);
            }
        }
        
        private static void PostProcessMaterial(LambertReflectionMaterial material)
        {
            if (material.R == 0)
            {
                material.R = 1;
            }
            material.R = Math.Clamp(material.R, 0, 1);
        }
        
        private static void PostProcessMaterial(SpecularReflectionMaterial material)
        {
            if (material.Eta == 0)
            {
                material.Eta = 1.5;
            }
            if (material.R == 0)
            {
                material.R = 1;
            }
            material.R = Math.Clamp(material.R, 0, 1);
        }
        
        private static void PostProcessMaterial(SpecularTransmitionMaterial material)
        {
            if (material.Eta == 0)
            {
                material.Eta = 1.5;
            }
            if (material.T == 0)
            {
                material.T = 1;
            }
            material.T = Math.Clamp(material.T, 0, 1);
        }
        
        private static void PostProcessMaterial(FresnelMaterial material)
        {
            if (material.Eta == 0)
            {
                material.Eta = 1.5;
            }
            if (material.R == 0)
            {
                material.R = 1;
            }
            if (material.T == 0)
            {
                material.T = 1;
            }
            material.R = Math.Clamp(material.R, 0, 1);
            material.T = Math.Clamp(material.T, 0, 1);
        }
        
        private static void PostProcessMaterial(OrenNayarMaterial material)
        {
            if (material.R == 0)
            {
                material.R = 1;
            }
            material.R = Math.Clamp(material.R, 0, 1);
            material.Roughness = Math.Clamp(material.Roughness, 0, 1);
        }
        
        private static void PostProcessMaterial(MicrofacetReflectionMaterial material)
        {
            if (material.Eta == 0)
            {
                material.Eta = 1.5;
            }
            if (material.R == 0)
            {
                material.R = 1;
            }
            material.R = Math.Clamp(material.R, 0, 1);
            material.Roughness = Math.Clamp(material.Roughness, 0, 1);
        }
        
        private static void PostProcessMaterial(MetalMaterial material)
        {
            if (material.Eta == 0)
            {
                material.Eta = 1.5;
            }
            if (material.K == 0)
            {
                material.K = 1;
            }
            if (material.R == 0)
            {
                material.R = 1;
            }
            material.R = Math.Clamp(material.R, 0, 1);
            material.Roughness = Math.Clamp(material.Roughness, 0, 1);
        }
        
        private static void PostProcessMaterial(PlasticMaterial material)
        {
            if (material.R == 0)
            {
                material.R = 1;
            }
            material.R = Math.Clamp(material.R, 0, 1);
            material.Roughness = Math.Clamp(material.Roughness, 0, 1);
        }
        
        private static void PostProcessMaterial(BlendMaterial material)
        {
            material.Roughness = Math.Clamp(material.Roughness, 0, 1);
        }

        private static void PostProcessTransform(Transform transform)
        {
            transform.Position ??= new Vector3 { };
            transform.Rotation ??= new Vector3 { };
            transform.Scale ??= new Vector3
            {
                X = 1,
                Y = 1,
                Z = 1,
            };
        }
        
        public static Scene Validate(Scene scene)
        {
            if (scene.Version != 1)
            {
                throw new SceneIOException("Unsupported SceneFormat version = " + scene.Version);
            }

            if (scene.RenderOptions != null)
            {
                ValidateRenderOptions(scene, scene.RenderOptions);
            }

            if (scene.SceneObjects != null)
            {
                ValidateSceneObjects(scene, scene.SceneObjects.ToList());
            }

            if (scene.Lights != null)
            {
                ValidateLights(scene, scene.Lights.ToList());
            }

            if (scene.Cameras != null)
            {
                ValidateCameras(scene, scene.Cameras.ToList());
            }

            if (scene.Materials != null)
            {
                ValidateMaterials(scene, scene.Materials.ToList());
            }
            
            return scene;
        }

        private static void ValidateRenderOptions(Scene scene, RenderOptions renderOptions)
        {
            if (FindCameraById(scene, renderOptions.CameraId) == null)
            {
                throw new SceneIOException("RenderOptions references camera with id = " + renderOptions.CameraId + " which is not present on the scene.");
            }

            if (renderOptions.Width <= 0)
            {
                throw new SceneIOException("RenderOptions width should be positive");
            }

            if (renderOptions.Height <= 0)
            {
                throw new SceneIOException("RenderOptions height should be positive");
            }
        }

        private static void ValidateSceneObjects(Scene scene, List<SceneObject> sceneObjects)
        {
            var ids = new HashSet<int>();
            
            foreach (var sceneObject in sceneObjects)
            {
                if (sceneObject.Id <= 0)
                {
                    throw new SceneIOException("SceneObject id should be equal or greater than 0, instead got: " +
                                               sceneObject.Id);
                }
                
                if (ids.Contains(sceneObject.Id))
                {
                    throw new SceneIOException("SceneObjects with duplicate id found = " + sceneObject.Id);
                }

                if (FindCameraById(scene, sceneObject.Id) != null)
                {
                    throw new Exception("Found SceneObject with invalid id = " + sceneObject.Id + ", this id is already used by camera");
                }

                if (FindLightById(scene, sceneObject.Id) != null)
                {
                    throw new Exception("Found SceneObject with invalid id = " + sceneObject.Id +
                                        ", this id is already used by light");
                }
                ids.Add(sceneObject.Id);

                ValidateSceneObject(scene, sceneObject);
            }
        }

        private static void ValidateSceneObject(Scene scene, SceneObject sceneObject)
        {
            if (sceneObject.Transform != null)
            {
                ValidateTransform(scene, sceneObject.Transform);
            }

            if (!string.IsNullOrEmpty(sceneObject.MaterialId))
            {
                if (FindMaterialById(scene, sceneObject.MaterialId) == null)
                {
                    throw new SceneIOException("Material with id=\"" + sceneObject.MaterialId + "\" is not present, but referenced by scene object with id=" + sceneObject.Id);
                }
            }

            if (sceneObject.Material != null)
            {
                ValidateMaterial(sceneObject.Material);
            }

            if (sceneObject.Sphere != null)
            {
                if (sceneObject.Sphere.Radius <= 0)
                {
                    throw new SceneIOException("Sphere should have a positive radius");
                }

                if (HasScale(sceneObject.Transform))
                {
                    throw new SceneIOException("Scale cannot be applied to Sphere, use radius instead.");
                }
            }

            if (sceneObject.Cube != null)
            {
                if (!IsNonNegative(sceneObject.Cube.Size))
                {
                    throw new SceneIOException("Cube size should be non-negative.");
                }

                if (HasScale(sceneObject.Transform))
                {
                    throw new SceneIOException("Scale cannot be applied to Cube, use size vector instead.");
                }
            }

            if (sceneObject.Disk != null)
            {
                if (sceneObject.Disk.Radius <= 0)
                {
                    throw new SceneIOException("Disk radius should be positive");
                }
            }

            if (sceneObject.MeshedObject != null)
            {
                if (sceneObject.MeshedObject.Reference == null)
                {
                    throw new SceneIOException("Reference not set for meshed object with id = " + sceneObject.Id);
                }
                
                if (!File.Exists(sceneObject.MeshedObject.Reference))
                {
                    throw new SceneIOException("MeshedObject references obj file which does not exist: " + sceneObject.MeshedObject.Reference);
                }
            }
        }

        private static void ValidateLights(Scene scene, List<Light> lights)
        {
            var ids = new HashSet<int>();
            foreach (var light in lights)
            {
                if (light.Id <= 0)
                {
                    throw new SceneIOException("Light id should be non-negative, got instead: " + light.Id);
                }
                
                if (ids.Contains(light.Id))
                {
                    throw new SceneIOException("Lights with duplicate id found: " + light.Id);
                }
                
                if (FindCameraById(scene, light.Id) != null)
                {
                    throw new Exception("Found Light with invalid id = " + light.Id + ", this id is already used by camera");
                }
                
                ValidateLight(scene, light);
            }
        }

        private static void ValidateLight(Scene scene, Light light)
        {
            if (light.Transform != null)
            {
                ValidateTransform(scene, light.Transform);
            }

            if (light.Color != null)
            {
                ValidateColor(light.Color);
            }

            if (HasScale(light.Transform))
            {
                throw new SceneIOException("Lights cannot have scale, but light with id = " + light.Id + " does have it.");
            }
        }

        private static void ValidateCameras(Scene scene, List<Camera> cameras)
        {
            var cameraIds = new HashSet<int>();
            
            foreach (var camera in scene.Cameras)
            {
                if (cameraIds.Contains(camera.Id))
                {
                    throw new SceneIOException("Cameras with duplicate ids found: " + camera.Id);
                }
                cameraIds.Add(camera.Id);

                if (camera.Transform != null)
                {
                    ValidateTransform(scene, camera.Transform);

                    if (HasScale(camera.Transform))
                    {
                        throw new SceneIOException("Camera cannot have scale, camera id = " + camera.Id);
                    }
                }

                if (camera.Perspective != null)
                {
                    if (camera.Perspective.Fov <= 0 || camera.Perspective.Fov >= 180)
                    {
                        throw new SceneIOException("Invalid FOV for camera, valid values are in range (0, 180).");
                    }
                }
            }
        }
        
        private static void ValidateTransform(Scene scene, Transform transform)
        {
            if (transform.ParentId != 0)
            {
                if (FindSceneObjectById(scene, transform.ParentId) == null &&
                    FindCameraById(scene, transform.ParentId) == null &&
                    FindLightById(scene, transform.ParentId) == null)
                {
                    throw new SceneIOException("Could not find parent transform by id: " + transform.ParentId);
                }

                if (!IsNonNegative(transform.Scale))
                {
                    throw new SceneIOException("Transform.Scale should be non-negative");
                }
            }
        }

        private static void ValidateMaterials(Scene scene, List<Material> materials)
        {
            foreach (var material in materials)
            {
                ValidateMaterial(material);
            }
        }

        private static void ValidateMaterial(Material material)
        {
            if (material.LambertReflection != null)
            {
                if (material.LambertReflection.Color == null)
                {
                    throw new SceneIOException($"Color should be set for LambertReflection material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.LambertReflection.Color);
            }
            if (material.SpecularReflection != null)
            {
                if (material.SpecularReflection.Color == null)
                {
                    throw new SceneIOException($"Color should be set for LambertReflection material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.SpecularReflection.Color);
            }
            if (material.SpecularTransmission != null)
            {
                if (material.SpecularTransmission.Color == null)
                {
                    throw new SceneIOException($"Color should be set for LambertReflection material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.SpecularTransmission.Color);
            }
            if (material.Fresnel != null)
            {
                if (material.Fresnel.Color == null)
                {
                    throw new SceneIOException($"Color should be set for LambertReflection material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.Fresnel.Color);
            }
            if (material.OrenNayar != null)
            {
                if (material.OrenNayar.Color == null)
                {
                    throw new SceneIOException($"Color should be set for OrenNayar material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.OrenNayar.Color);
            }
            if (material.MicrofacetReflection != null)
            {
                if (material.MicrofacetReflection.Color == null)
                {
                    throw new SceneIOException($"Color should be set for MicrofacetReflection material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.MicrofacetReflection.Color);
            }
            if (material.Metal != null)
            {
                if (material.Metal.Color == null)
                {
                    throw new SceneIOException($"Color should be set for Metal material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.Metal.Color);
            }
            if (material.Plastic != null)
            {
                if (material.Plastic.Color == null)
                {
                    throw new SceneIOException($"Color should be set for Plastic material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.Plastic.Color);
            }
            if (material.Blend != null)
            {
                if (material.Blend.Diffuse == null || material.Blend.Specular == null)
                {
                    throw new SceneIOException($"Color should be set for Blend material.{GetMaterialIdMessage(material)}");
                }
                ValidateColor(material.Blend.Diffuse);
                ValidateColor(material.Blend.Specular);
            }
        }

        private static string GetMaterialIdMessage(Material material)
        {
            return string.IsNullOrEmpty(material.Id) ? "" : $"Material id ={material.Id}";
        }

        private static void ValidateColor(Color color)
        {
            if (color.R < 0 || color.G < 0 || color.B < 0)
            {
                throw new SceneIOException("Color channels should be non-negative");
            }
        }
        
        private static Camera FindCameraById(Scene scene, int cameraId)
        {
            foreach (var camera in scene.Cameras)
            {
                if (camera.Id == cameraId)
                {
                    return camera;
                }
            }

            return null;
        }
        
        private static Light FindLightById(Scene scene, int lightId)
        {
            foreach (var light in scene.Lights)
            {
                if (light.Id == lightId)
                {
                    return light;
                }
            }

            return null;
        }

        private static SceneObject FindSceneObjectById(Scene scene, int sceneObjectId)
        {
            foreach (var sceneObject in scene.SceneObjects)
            {
                if (sceneObject.Id == sceneObjectId)
                {
                    return sceneObject;
                }
            }

            return null;
        }

        private static Material FindMaterialById(Scene scene, string materialId)
        {
            foreach (var material in scene.Materials)
            {
                if (material.Id == materialId)
                {
                    return material;
                }
            }

            return null;
        }

        private static bool IsNonNegative(Vector3 v)
        {
            return v.X >= 0 && v.Y >= 0 && v.Z >= 0;
        }
        
        private static bool HasScale(Transform transform)
        {
            if (transform.Scale == null)
            {
                return false;
            }

            return Math.Abs(transform.Scale.X - 1) > TOLERANCE || Math.Abs(transform.Scale.Y - 1) > TOLERANCE 
                                                               || Math.Abs(transform.Scale.Z - 1) > TOLERANCE;
        }
    }
}