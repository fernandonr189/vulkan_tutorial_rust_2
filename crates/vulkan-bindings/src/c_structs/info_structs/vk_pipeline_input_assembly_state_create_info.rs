use crate::{
    VkPipelineInputAssemblyStateCreateInfo, VkPrimitiveTopology,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    create_info_builder,
};

create_info_builder!(
    VkPipelineInputAssemblyStateCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    set_topology => topology: VkPrimitiveTopology,
    set_primitive_restart_enable => primitiveRestartEnable: u32,
);
