use crate::{
    VkShaderModuleCreateInfo, VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
    create_info_builder,
};

create_info_builder!(
    VkShaderModuleCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
    set_code_size => codeSize: usize,
    set_p_code => pCode: *const u32 => null,
);
