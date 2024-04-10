#import bevy_sprite::{
        // https://github.com/bevyengine/bevy/blob/release-0.13.0/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl
        mesh2d_vertex_output::VertexOutput,
        mesh2d_view_bindings::view,
}

struct CustomMaterial {
        color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: CustomMaterial;
@group(2) @binding(1) var           texture: texture_2d<f32>;
@group(2) @binding(2) var   texture_sampler: sampler;

fn sobel_operator(texture: texture_2d<f32>, texture_sampler: sampler, uv: vec2<f32>) -> f32 {
    //  used to calculate the horizontal and vertical size of a single pixel in texture space
    let dX = 1.0 / f32(textureDimensions(texture, 0).x);
    let dY = 1.0 / f32(textureDimensions(texture, 0).y);

    // Sample the surrounding pixels
    let topLeft =     textureSample(texture, texture_sampler, uv + vec2<f32>(-dX, -dY)).r;
    let top =         textureSample(texture, texture_sampler, uv + vec2<f32>(0.0, -dY)).r;
    let topRight =    textureSample(texture, texture_sampler, uv + vec2<f32>(dX, -dY)).r;
    let left =        textureSample(texture, texture_sampler, uv + vec2<f32>(-dX, 0.0)).r;
    let center =      textureSample(texture, texture_sampler, uv + vec2<f32>(0.0, 0.0)).r;
    let right =       textureSample(texture, texture_sampler, uv + vec2<f32>(dX, 0.0)).r;
    let bottomLeft =  textureSample(texture, texture_sampler, uv + vec2<f32>(-dX, dY)).r;
    let bottom =      textureSample(texture, texture_sampler, uv + vec2<f32>(0.0, dY)).r;
    let bottomRight = textureSample(texture, texture_sampler, uv + vec2<f32>(dX, dY)).r;

    // Apply the Sobel X kernel
    var gradX = topLeft * -1.0 + top * 0.0 + topRight * 1.0 +
                left * -2.0 + center * 0.0 + right * 2.0 +
                bottomLeft * -1.0 + bottom * 0.0 + bottomRight * 1.0;

    // Apply the Sobel Y kernel
    var gradY = topLeft * -1.0 + top * -2.0 + topRight * -1.0 +
                left * 0.0 + center * 0.0 + right * 0.0 +
                bottomLeft * 1.0 + bottom * 2.0 + bottomRight * 1.0;

    // Compute the magnitude of the gradient (edge strength)
    let edgeStrength = sqrt(gradX * gradX + gradY * gradY);
    return edgeStrength;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
        // Output edge strength as a grayscale value
        // let edge_strength = sobel_operator(texture, texture_sampler, mesh.uv);
        // Set a threshold for what is considered an 'edge'
        // let threshold: f32 = 0.1; // Adjust this value as needed for your specific texture/content

        // let alpha: f32 = textureSample(texture, texture_sampler, mesh.uv).a;
        
        // // // 255 == 1.0
        // // // f32(textureDimensions(texture, 0).y == 150.0 && // f32(textureDimensions(texture, 0).x == 150.0
        // if (f32(textureDimensions(texture, 0).y) == 150.0) {
        //         // Edge: Color it black
        //         return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        // } else {
        //         // Not an edge: Color it white
        //         return vec4<f32>(250.0, 250.0, 250.0, 1.0);
        // }
        let center: vec2<f32> = vec2<f32>(0.5, 0.5);
        let radius: f32 = 0.5;
        let borderThickness: f32 = 0.05;
        let innerColor: vec4<f32> = vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red
        let borderColor: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Black
        
        let distanceToCenter: f32 = length((mesh.position.xy / mesh.position.w) - center);
        if (distanceToCenter < radius) {
                if (distanceToCenter > radius - borderThickness) {
                        return borderColor;
                }
                return innerColor;
        }
        discard;

}