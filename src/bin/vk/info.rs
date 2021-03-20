use vulkano::device::Features;
use vulkano::instance::PhysicalDevice;

pub fn print_pd(pd: &PhysicalDevice) {
    println!("Name           : {}", pd.name());
    println!("Type           : {:?}", pd.ty());
    println!("Api version    : {}", pd.api_version());
    println!("Driver version : {}", pd.driver_version());
    println!("PCI Device ID  : {}", pd.pci_device_id());
    println!("PCI Vender ID  : {}", pd.pci_vendor_id());
    println!("UUID           : {}", uuid::Uuid::from_bytes(*pd.uuid()));

    println!("Memory types");
    println!("  id    local    cached    coherent   visible   lazy   size");
    for mt in pd.memory_types() {
        println!(
            "{:4} {:>8} {:>9} {:>11} {:>9} {:>6} {:>6}",
            mt.id(),
            mt.is_device_local(),
            mt.is_host_cached(),
            mt.is_host_coherent(),
            mt.is_host_visible(),
            mt.is_lazily_allocated(),
            (mt.heap().size() as f64) / 1024.0 / 1024.0,
        )
    }

    println!("Memory heaps");
    println!("  id    size");
    for mh in pd.memory_heaps() {
        println!("{:4} {:>7}", mh.id(), (mh.size() as f64) / 1024.0 / 1024.0,)
    }

    println!("Queue families");
    println!("  id  count  ts  image-whd  graphics  compute transfers sparse");
    for qf in pd.queue_families() {
        let whd = qf
            .min_image_transfer_granularity()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("x");
        println!(
            "{:4} {:>6} {:>3} {:>8} {:>11} {:>8} {:>9} {:>6}",
            qf.id(),
            qf.queues_count(),
            qf.timestamp_valid_bits().unwrap_or(0),
            whd,
            qf.supports_graphics(),
            qf.supports_compute(),
            qf.explicitly_supports_transfers(),
            qf.supports_sparse_binding(),
        )
    }
}

pub fn print_features(pd: &PhysicalDevice, features: &Features) {
    let items = vec![
        ("robust_buffer_access", features.robust_buffer_access),
        ("full_draw_index_uint32", features.full_draw_index_uint32),
        ("image_cube_array", features.image_cube_array),
        ("independent_blend", features.independent_blend),
        ("geometry_shader", features.geometry_shader),
        ("tessellation_shader", features.tessellation_shader),
        ("sample_rate_shading", features.sample_rate_shading),
        ("dual_src_blend", features.dual_src_blend),
        ("logic_op", features.logic_op),
        ("multi_draw_indirect", features.multi_draw_indirect),
        (
            "draw_indirect_first_instance",
            features.draw_indirect_first_instance,
        ),
        ("depth_clamp", features.depth_clamp),
        ("depth_bias_clamp", features.depth_bias_clamp),
        ("fill_mode_non_solid", features.fill_mode_non_solid),
        ("depth_bounds", features.depth_bounds),
        ("wide_lines", features.wide_lines),
        ("large_points", features.large_points),
        ("alpha_to_one", features.alpha_to_one),
        ("multi_viewport", features.multi_viewport),
        ("sampler_anisotropy", features.sampler_anisotropy),
        (
            "texture_compression_etc2",
            features.texture_compression_etc2,
        ),
        (
            "texture_compression_astc_ldr",
            features.texture_compression_astc_ldr,
        ),
        ("texture_compression_bc", features.texture_compression_bc),
        ("occlusion_query_precise", features.occlusion_query_precise),
        (
            "pipeline_statistics_query",
            features.pipeline_statistics_query,
        ),
        (
            "vertex_pipeline_stores_and_atomics",
            features.vertex_pipeline_stores_and_atomics,
        ),
        (
            "fragment_stores_and_atomics",
            features.fragment_stores_and_atomics,
        ),
        (
            "shader_tessellation_and_geometry_point_size",
            features.shader_tessellation_and_geometry_point_size,
        ),
        (
            "shader_image_gather_extended",
            features.shader_image_gather_extended,
        ),
        (
            "shader_storage_image_extended_formats",
            features.shader_storage_image_extended_formats,
        ),
        (
            "shader_storage_image_multisample",
            features.shader_storage_image_multisample,
        ),
        (
            "shader_storage_image_read_without_format",
            features.shader_storage_image_read_without_format,
        ),
        (
            "shader_storage_image_write_without_format",
            features.shader_storage_image_write_without_format,
        ),
        (
            "shader_uniform_buffer_array_dynamic_indexing",
            features.shader_uniform_buffer_array_dynamic_indexing,
        ),
        (
            "shader_sampled_image_array_dynamic_indexing",
            features.shader_sampled_image_array_dynamic_indexing,
        ),
        (
            "shader_storage_buffer_array_dynamic_indexing",
            features.shader_storage_buffer_array_dynamic_indexing,
        ),
        (
            "shader_storage_image_array_dynamic_indexing",
            features.shader_storage_image_array_dynamic_indexing,
        ),
        ("shader_clip_distance", features.shader_clip_distance),
        ("shader_cull_distance", features.shader_cull_distance),
        ("shader_float64", features.shader_float64),
        ("shader_int64", features.shader_int64),
        ("shader_int16", features.shader_int16),
        (
            "shader_resource_residency",
            features.shader_resource_residency,
        ),
        ("shader_resource_min_lod", features.shader_resource_min_lod),
        ("sparse_binding", features.sparse_binding),
        ("sparse_residency_buffer", features.sparse_residency_buffer),
        (
            "sparse_residency_image2d",
            features.sparse_residency_image2d,
        ),
        (
            "sparse_residency_image3d",
            features.sparse_residency_image3d,
        ),
        (
            "sparse_residency2_samples",
            features.sparse_residency2_samples,
        ),
        (
            "sparse_residency4_samples",
            features.sparse_residency4_samples,
        ),
        (
            "sparse_residency8_samples",
            features.sparse_residency8_samples,
        ),
        (
            "sparse_residency16_samples",
            features.sparse_residency16_samples,
        ),
        (
            "sparse_residency_aliased",
            features.sparse_residency_aliased,
        ),
        (
            "variable_multisample_rate",
            features.variable_multisample_rate,
        ),
        ("inherited_queries", features.inherited_queries),
        ("buffer_device_address", features.buffer_device_address),
        (
            "buffer_device_address_capture_replay",
            features.buffer_device_address_capture_replay,
        ),
        (
            "buffer_device_address_multi_device",
            features.buffer_device_address_multi_device,
        ),
        (
            "variable_pointers_storage_buffer",
            features.variable_pointers_storage_buffer,
        ),
        ("variable_pointers", features.variable_pointers),
        (
            "shader_buffer_int64_atomics",
            features.shader_buffer_int64_atomics,
        ),
        (
            "shader_shared_int64_atomics",
            features.shader_shared_int64_atomics,
        ),
        ("storage_buffer_8bit", features.storage_buffer_8bit),
        ("storage_uniform_8bit", features.storage_uniform_8bit),
        (
            "storage_push_constant_8bit",
            features.storage_push_constant_8bit,
        ),
        ("storage_buffer_16bit", features.storage_buffer_16bit),
        ("storage_uniform_16bit", features.storage_uniform_16bit),
        (
            "storage_push_constant_16bit",
            features.storage_push_constant_16bit,
        ),
        (
            "storage_input_output_16bit",
            features.storage_input_output_16bit,
        ),
        ("shader_float16", features.shader_float16),
        ("shader_int8", features.shader_int8),
    ];

    println!("device-name : {}", pd.name());
    println!("features    :");
    for (name, featr) in items.into_iter() {
        if featr {
            println!("    {}", name);
        }
    }
}
