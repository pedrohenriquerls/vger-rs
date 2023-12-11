use std::collections::HashMap;

use crate::*;

pub const MAX_LAYERS: usize = 4;

type Mat4x4 = [f32; 16];

pub(crate) struct Scene {
    pub depthed_prims: HashMap<i32, Vec<Prim>>,
    pub prims: [GPUVec<Prim>; MAX_LAYERS],
    pub cvs: GPUVec<LocalPoint>,
    pub xforms: GPUVec<Mat4x4>,
    pub paints: GPUVec<Paint>,
    pub scissors: GPUVec<Scissor>,
    //pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_groups: [wgpu::BindGroup; MAX_LAYERS],
}

pub const MAX_PRIMS: usize = 65536;

impl Scene {
    pub fn new(device: &wgpu::Device) -> Self {
        let prims = [
            GPUVec::new(device, MAX_PRIMS, "Prim Buffer 0"),
            GPUVec::new(device, MAX_PRIMS, "Prim Buffer 1"),
            GPUVec::new(device, MAX_PRIMS, "Prim Buffer 2"),
            GPUVec::new(device, MAX_PRIMS, "Prim Buffer 3"),
        ];

        let cvs = GPUVec::new(device, MAX_PRIMS, "cv Buffer");
        let xforms = GPUVec::new(device, MAX_PRIMS, "Xform Buffer");
        let paints = GPUVec::new(device, MAX_PRIMS, "Paint Buffer");
        let scissors = GPUVec::new(device, MAX_PRIMS, "scissor Buffer");

        //let bind_group_layout = Self::bind_group_layout(device);

        let bind_groups = [
            Scene::bind_group(device, &prims[0], &cvs, &xforms, &paints, &scissors),
            Scene::bind_group(device, &prims[1], &cvs, &xforms, &paints, &scissors),
            Scene::bind_group(device, &prims[2], &cvs, &xforms, &paints, &scissors),
            Scene::bind_group(device, &prims[3], &cvs, &xforms, &paints, &scissors),
        ];

        Self {
            depthed_prims: HashMap::new(),
            prims,
            cvs,
            xforms,
            paints,
            scissors,
            //bind_group_layout,
            bind_groups,
        }
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                GPUVec::<Prim>::bind_group_layout_entry(0),
                GPUVec::<LocalPoint>::bind_group_layout_entry(1),
                GPUVec::<Mat4x4>::bind_group_layout_entry(2),
                GPUVec::<Paint>::bind_group_layout_entry(3),
                GPUVec::<Scissor>::bind_group_layout_entry(4),
            ],
            label: Some("bind_group_layout"),
        })
    }

    fn bind_group(
        device: &wgpu::Device,
        prims: &GPUVec<Prim>,
        cvs: &GPUVec<LocalPoint>,
        xforms: &GPUVec<Mat4x4>,
        paints: &GPUVec<Paint>,
        scissors: &GPUVec<Scissor>,
    ) -> wgpu::BindGroup {
        let bind_group_layout = Self::bind_group_layout(device);

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                prims.bind_group_entry(0),
                cvs.bind_group_entry(1),
                xforms.bind_group_entry(2),
                paints.bind_group_entry(3),
                scissors.bind_group_entry(4),
            ],
            label: Some("vger bind group"),
        })
    }

    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut keys: Vec<i32> = self.depthed_prims.keys().copied().collect();
        keys.sort();
        for z_index in keys {
            let mut prims = self.depthed_prims.remove(&z_index).unwrap();
            self.prims[0].append(&mut prims);
        }

        for i in 0..4 {
            self.prims[i].update(device, queue);
        }
        self.cvs.update(device, queue);
        self.xforms.update(device, queue);
        self.paints.update(device, queue);
        self.scissors.update(device, queue);
    }

    pub fn clear(&mut self) {
        self.depthed_prims.clear();
        for i in 0..4 {
            self.prims[i].clear();
        }
        self.cvs.clear();
        self.xforms.clear();
        self.paints.clear();
        self.scissors.clear();
    }
}
