use artie_common::structure::{Workspace, Block, Input, Field, ArtieDistance};
use crate::pb::artie_distances::{
    Workspace as GrpcWorkspace, Block as GrpcBlock, Input as GrpcInput, Field as GrpcField,
    ArtieDistance as GrpcArtieDistance,
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
impl From<ArtieDistance> for GrpcArtieDistance {
    fn from(artie_distance: ArtieDistance) -> Self {
        GrpcArtieDistance {
            family_distance: artie_distance.family_distance,
            block_distance: artie_distance.block_distance,
            position_distance: artie_distance.position_distance,
            input_distance: artie_distance.input_distance,
            total_distance: artie_distance.total_distance,
            workspace_adjustments: None,
        }
    }
}