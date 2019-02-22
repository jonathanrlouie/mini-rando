use std::{
    error::Error,
    fmt,
    fs::File,
    path::Path
};
use tiled;
use num_traits::{CheckedRem, CheckedSub, CheckedDiv};

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

pub struct TilemapInfo {
    tilemap_dims: TilemapDimensions,
    tileset_dims: TilesetDimensions,
    tilemap_tiles: TilemapTiles
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

/*
impl Component for TilemapTiles {
    type Storage = DenseVecStorage<Self>;
}*/

// TODO: smart constructor this bad boi
struct ImageSource(String);

pub fn initialise_tilemap() -> Result<TilemapInfo, Box<Error>> {
    let map = parse_tmx_file()?;
    let tileset = fetch_tileset(&map)?;
    let img = fetch_tileset_image(tileset)?;
    let tilemap_dims = TilemapDimensions {
        width: map.width,
        height: map.height
    };
    let tileset_dims = fetch_tileset_dims(tileset, img)?;
    let img_src = fetch_img_src(img);
    let tilemap_tiles = generate_tile_data(&map, &tileset_dims)?;
    Ok(TilemapInfo { tilemap_dims, tileset_dims, tilemap_tiles })
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

fn parse_tmx_file() -> Result<tiled::Map, Box<Error>> {
    let map_file = File::open(&Path::new("./resources/tetris_tilemap.tmx"))?;
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