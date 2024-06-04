use artie_common::structure::{hint::{BlockChange, BlockPositionChange, InputChange, WorkspaceAdjustments}, ArtieDistance, Block, Field, Input, Workspace};
use crate::pb::artie_distances::{
    Workspace as GrpcWorkspace, Block as GrpcBlock, Input as GrpcInput, Field as GrpcField,
    ArtieDistance as GrpcArtieDistance,
    BlockChange as GrpcBlockChange, BlockPositionChange as GrpcBlockPositionChange, InputChange as GrpcInputChange,
    WorkspaceAdjustments as GrpcWorkspaceAdjustments,
};

// Implement conversion from GrpcWorkspace to internal Workspace
impl From<GrpcField> for Field {
    fn from(grcp_field: GrpcField) -> Self {
        Field {
            name: grcp_field.name,
            value: grcp_field.value,
        }
    }
}

impl From<GrpcInput> for Input {
    fn from(grpc_input: GrpcInput) -> Self {
        Input {
            name: grpc_input.name,
            code: grpc_input.code,
            fields: grpc_input.fields.into_iter().map(Field::from).collect(),
        }
    }
}

impl From<GrpcBlock> for Block {
    fn from(grpc_block: GrpcBlock) -> Self {
        Block {
            id: grpc_block.id,
            name: grpc_block.name,
            family: grpc_block.family,
            inputs: grpc_block.inputs.into_iter().map(Input::from).collect(),
            next: grpc_block.next.map(|b| Box::new(Block::from(*b))),
            nested: grpc_block.nested.into_iter().map(Block::from).collect(),
            parent: None,
            previous: None,
        }
    }
}

impl From<GrpcWorkspace> for Workspace {
    fn from(grpc_workspace: GrpcWorkspace) -> Self {
        Workspace {
            id: grpc_workspace.id,
            name: grpc_workspace.name,
            blocks: grpc_workspace.blocks.into_iter().map(Block::from).collect(),
        }
    }
}


// Implement conversion from internal Workspace to GrpcWorkspace
impl From<Field> for GrpcField {
    fn from(field: Field) -> Self {
        GrpcField {
            name: field.name,
            value: field.value,
        }
    }
}

impl From<Input> for GrpcInput {
    fn from(input: Input) -> Self {
        GrpcInput {
            name: input.name,
            code: input.code,
            fields: input.fields.into_iter().map(GrpcField::from).collect(),
        }
    }
}

impl From<Block> for GrpcBlock {
    fn from(block: Block) -> Self {
        GrpcBlock {
            id: block.id,
            name: block.name,
            family: block.family,
            inputs: block.inputs.into_iter().map(GrpcInput::from).collect(),
            next: block.next.map(|b| Box::new(GrpcBlock::from(*b))),
            previous: block.previous.map(|b| Box::new(GrpcBlock::from(*b))),
            parent: block.parent.map(|b| Box::new(GrpcBlock::from(*b))),
            nested: block.nested.into_iter().map(GrpcBlock::from).collect(),
        }
    }
}

impl From<Workspace> for GrpcWorkspace {
    fn from(workspace: Workspace) -> Self {
        GrpcWorkspace {
            id: workspace.id,
            name: workspace.name,
            blocks: workspace.blocks.into_iter().map(GrpcBlock::from).collect(),
        }
    }
}


// TODO: Implement the workspace_adjustments conversion
impl From<BlockChange> for GrpcBlockChange {
    fn from(block_change: BlockChange) -> Self {
        GrpcBlockChange {
            id: block_change.id,
            name: block_change.name,
        }
    }
}

impl From<BlockPositionChange> for GrpcBlockPositionChange {
    fn from(block_position_change: BlockPositionChange) -> Self {
        GrpcBlockPositionChange {
            block: Some(GrpcBlockChange::from(block_position_change.block)),
            current_position: block_position_change.current_position.into_iter().map(|x| x as u32).collect(),
            target_position: block_position_change.target_position.into_iter().map(|x| x as u32).collect(),
        }
    }
}

impl From<InputChange> for GrpcInputChange {
    fn from(input_change: InputChange) -> Self {
        GrpcInputChange {
            block_id: input_change.block_id,
            input_name: input_change.input_name,
            expected_value: input_change.expected_value,
            actual_value: input_change.actual_value,
        }
    }
}

impl From<WorkspaceAdjustments> for GrpcWorkspaceAdjustments {
    fn from(workspace_adjustments: WorkspaceAdjustments) -> Self {
        GrpcWorkspaceAdjustments {
            blocks_to_remove: workspace_adjustments.blocks_to_remove.into_iter().map(GrpcBlockChange::from).collect(),
            blocks_to_add: workspace_adjustments.blocks_to_add.into_iter().map(GrpcBlockChange::from).collect(),
            blocks_to_reposition: workspace_adjustments.blocks_to_reposition.into_iter().map(GrpcBlockPositionChange::from).collect(),
            blocks_with_input_changes: workspace_adjustments.blocks_with_input_changes.into_iter().map(GrpcInputChange::from).collect(),
        }
    }
}

impl From<ArtieDistance> for GrpcArtieDistance {
    fn from(artie_distance: ArtieDistance) -> Self {
        GrpcArtieDistance {
            family_distance: artie_distance.family_distance,
            block_distance: artie_distance.block_distance,
            position_distance: artie_distance.position_distance,
            input_distance: artie_distance.input_distance,
            total_distance: artie_distance.total_distance,
            workspace_adjustments: Some(GrpcWorkspaceAdjustments::from(artie_distance.workspace_adjustments)),
        }
    }
}