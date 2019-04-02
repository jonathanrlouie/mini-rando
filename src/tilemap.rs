use std::{
    error::Error,
    fmt,
    fs::File,
    path::Path
};
use tiled;
use num_traits::{CheckedSub, CheckedDiv, CheckedMul};
use amethyst::{
    core::nalgebra::{Vector2, Vector3},
    renderer::PosTex
};
use genmesh::{
    Triangulate, Vertices,
    generators::{Plane, SharedVertex, IndexedPolygon}
};

#[derive(Debug)]
pub struct FetchTilesetError;

impl Error for FetchTilesetError {}

impl fmt::Display for FetchTilesetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error fetching tileset")
    }
}

#[derive(Debug)]
pub struct FetchImageError;

impl Error for FetchImageError {}

impl fmt::Display for FetchImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error fetching image")
    }
}

#[derive(Debug)]
pub struct TileDataError;

impl Error for TileDataError {}

impl fmt::Display for TileDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error generating tile data")
    }
}

#[derive(Debug)]
pub struct FetchTilesetWidthError;

impl Error for FetchTilesetWidthError {}

impl fmt::Display for FetchTilesetWidthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error fetching tileset width")
    }
}

#[derive(Debug)]
pub struct FetchTilesetHeightError;

impl Error for FetchTilesetHeightError {}

impl fmt::Display for FetchTilesetHeightError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error fetching tileset height")
    }
}

#[derive(Debug)]
pub struct TilemapWidthTooLargeError;

impl Error for TilemapWidthTooLargeError {}

impl fmt::Display for TilemapWidthTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tilemap width is too large")
    }
}

#[derive(Debug)]
pub struct TilemapHeightTooLargeError;

impl Error for TilemapHeightTooLargeError {}

impl fmt::Display for TilemapHeightTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tilemap height is too large")
    }
}

#[derive(Debug)]
pub struct CouldNotFindVertexDataError(usize);

impl Error for CouldNotFindVertexDataError {}

impl fmt::Display for CouldNotFindVertexDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find vertex data at index: {}", self.0)
    }
}

pub struct TilemapPath<'a> {
    path: &'a str
}

pub struct TilemapInfo {
    tilemap_dims: TilemapDimensions,
    tileset_dims: TilesetDimensions,
    tilemap_tiles: TilemapTiles,
    img_src: ImageSource
}

pub struct TilemapDimensions {
    width: u32,
    height: u32
}

pub struct TilesetDimensions {
    width: u32,
    height: u32
}

#[derive(Clone)]
pub struct TilemapTiles {
    pub tiles: Vec<[f32; 4]>,
}

trait TiledMap<Tileset, Layer> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn tilesets(self) -> Vec<Tileset>;
    fn layers(self) -> Vec<Layer>;
}

impl TiledMap<tiled::Tileset, tiled::Layer> for tiled::Map {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn tilesets(self) -> Vec<tiled::Tileset> {
        self.tilesets
    }

    fn layers(self) -> Vec<tiled::Layer> {
        self.layers
    }
}

struct ImageSource(String);

pub fn initialise_tilemap(tilemap_path: TilemapPath) -> Result<TilemapInfo, Box<Error>> {
    let map = parse_tmx_file(tilemap_path)?;
    let tileset = fetch_tileset(&map)?;
    let img = fetch_tileset_image(tileset)?;
    let tilemap_dims = TilemapDimensions {
        width: map.width,
        height: map.height
    };
    let tileset_dims = fetch_tileset_dims(tileset, img)?;
    let img_src = fetch_img_src(img);
    let tilemap_tiles = generate_tile_data(&map, &tileset_dims)?;
    Ok(TilemapInfo { tilemap_dims, tileset_dims, tilemap_tiles, img_src })
}

fn fetch_tileset_dims(tileset: &tiled::Tileset, img: &tiled::Image) -> Result<TilesetDimensions, Box<Error>> {
    let tileset_width = CheckedDiv::checked_div(&(img.width as u32), &tileset.tile_width)
        .ok_or_else(|| FetchTilesetWidthError)?;
    let tileset_height = CheckedDiv::checked_div(&(img.height as u32), &tileset.tile_height)
        .ok_or_else(|| FetchTilesetHeightError)?;

    Ok(TilesetDimensions {
        width: tileset_width,
        height: tileset_height
    })
}

fn fetch_img_src(img: &tiled::Image) -> ImageSource {
    ImageSource(format!("{}{}", "../resources/", &img.source))
}

// Example path: "./resources/tetris_tilemap.tmx"
fn parse_tmx_file(tilemap_path: TilemapPath) -> Result<tiled::Map, Box<Error>> {
    let map_file = File::open(&Path::new(tilemap_path.path))?;
    let map = tiled::parse(map_file)?;
    Ok(map)
}

fn fetch_tileset(map: &tiled::Map) -> Result<&tiled::Tileset, FetchTilesetError> {
    map.tilesets.get(0).ok_or_else(|| FetchTilesetError)
}

fn fetch_tileset_image(tileset: &tiled::Tileset) -> Result<&tiled::Image, FetchImageError> {
    tileset.images.get(0).ok_or_else(|| FetchImageError)
}

/*
A Tiled map looks somewhat like this:
[0, 0, 0, 0, 0, 0,
1, 0, 0 ,0 ,0 ,0
2, 0, 3, 1, 1, 0]
*/
fn generate_tile_data(map: &tiled::Map, tileset_dimensions: &TilesetDimensions) -> Result<TilemapTiles, TileDataError> {
    let &TilesetDimensions{ width, height } = tileset_dimensions;
    map.layers.iter()
        .flat_map(|layer| layer.tiles.iter()
            .flat_map(|rows| rows.into_iter()
                .map(|&tile| {
                    if tile != 0 {
                        calculate_tile_data(tile, width, height)
                    } else {
                        Some([0.0, 0.0, 0.0, 0.0])
                    }
                })))
        .collect::<Option<Vec<[f32; 4]>>>()
        .ok_or_else(|| TileDataError)
        .map(|tiles| TilemapTiles { tiles })
}

fn calculate_tile_data(tile: u32, width: u32, height: u32) -> Option<[f32; 4]> {
    let tile_sub1 = CheckedSub::checked_sub(&tile, &1)?;
    let x = tile_sub1 as f32 % (width as f32);

    let height_sub1 = CheckedSub::checked_sub(&height, &1).map(|h| h as f32)?;
    let tile_over_width = CheckedDiv::checked_div(&tile_sub1, &width).map(|tow| tow as f32)?;
    let y = height_sub1 - tile_over_width;
    Some([x, y, 0.0, 0.0])
}

pub fn generate_tilemap_plane(tilesize: u32, tilemap_width: u32, tilemap_height: u32) -> Result<Vec<PosTex>, Box<Error>> {
    let plane = Plane::subdivide(tilemap_width as usize, tilemap_height as usize);

    let total_width = CheckedMul::checked_mul(&tilesize, &tilemap_width)
        .ok_or_else(|| TilemapWidthTooLargeError)?;
    let total_height = CheckedMul::checked_mul(&tilesize, &tilemap_height)
        .ok_or_else(|| TilemapHeightTooLargeError)?;

    let half_width = total_width as f32 / 2.0 ;
    let half_height = total_height as f32 / 2.0;

    let vertex_data: Vec<PosTex> = plane.shared_vertex_iter().map(|(raw_x, raw_y)| {
        let vertex_x = (half_width * raw_x).round();
        let vertex_y = (half_height * raw_y).round();

        let u_pos = (1.0 + raw_x) / 2.0;
        let v_pos = (1.0 + raw_y) / 2.0;

        let tilemap_x = (u_pos * tilemap_width as f32).round();
        let tilemap_y = (v_pos * tilemap_height as f32).round();

        PosTex {
            position: Vector3::new(vertex_x, vertex_y, 0.0),
            tex_coord: Vector2::new(tilemap_x, tilemap_height as f32 - tilemap_y)
        }
    }).collect();

    let indexed_vertex_data = plane.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| vertex_data.get(i as usize)
            .map(|&data| data)
            .ok_or_else(|| CouldNotFindVertexDataError(i)))
        .collect::<Result<Vec<PosTex>, CouldNotFindVertexDataError>>()?;

    Ok(indexed_vertex_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTiledMap<Tileset, Layer> {
        width: u32,
        height: u32,
        layers: Vec<Layer>,
        tilesets: Vec<Tileset>
    }

    impl<Tileset, Layer> TiledMap<Tileset, Layer> for MockTiledMap<Tileset, Layer> {
        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn tilesets(self) -> Vec<Tileset> {
            self.tilesets
        }

        fn layers(self) -> Vec<Layer> {
            self.layers
        }
    }

    #[test]
    fn tilemap_info_test() {
        let mock = MockTiledMap::<u32, u32> {
            width: 64,
            height: 64,
            layers: Vec::new(),
            tilesets: Vec::new()
        };
        assert_eq!(true, true)
    }
}