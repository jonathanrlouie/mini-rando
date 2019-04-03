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

pub struct TilemapInfo<TImageSource> {
    tilemap_dims: TilemapDimensions,
    tileset_dims: TilesetDimensions,
    tilemap_tiles: TilemapTiles,
    img_src: TImageSource
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

trait TilemapCreator<
    'creator,
    TImageSource,
    TImage: TiledImage<TImageSource>,
    TTileset: TiledTileset<TImageSource, TImage>,
    TLayer: TiledLayer,
    TMap: TiledMap< TImageSource, TImage, TTileset, TLayer>
> {
    fn create_map(&'creator self) -> Result<TMap, Box<Error>>;

    fn initialise_tilemap(&'creator self) -> Result<TilemapInfo<TImageSource>, Box<Error>> {
        let map = self.create_map()?;
        let tileset = map.fetch_tileset()?;
        let img = tileset.fetch_tileset_image()?;
        let tilemap_dims = TilemapDimensions {
            width: map.width(),
            height: map.height()
        };
        let tileset_dims = tileset.fetch_tileset_dims(img)?;
        let img_src = img.fetch_img_src();
        let tilemap_tiles = map.generate_tile_data(&tileset_dims)?;
        Ok(TilemapInfo::<TImageSource> { tilemap_dims, tileset_dims, tilemap_tiles, img_src })
    }
}

impl<'a> TilemapCreator<'a, ImageSource, tiled::Image, tiled::Tileset, tiled::Layer, tiled::Map> for TilemapPath<'a> {
    // Example path: "./resources/tetris_tilemap.tmx"
    fn create_map(&self) -> Result<tiled::Map, Box<Error>> {
        let map_file = File::open(&Path::new(self.path))?;
        let map = tiled::parse(map_file)?;
        Ok(map)
    }
}

// My tiled types

trait TiledMap<
    TImageSource,
    TImage: TiledImage<TImageSource>,
    TTileset: TiledTileset<TImageSource, TImage>,
    TLayer: TiledLayer
> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn tilesets(&self) -> &Vec<TTileset>;
    fn layers(&self) -> &Vec<TLayer>;
    fn fetch_tileset(&self) -> Result<&TTileset, FetchTilesetError> where Self: Sized {
        self.tilesets().get(0).ok_or_else(|| FetchTilesetError)
    }

    /*
    A Tiled map looks somewhat like this:
    [0, 0, 0, 0, 0, 0,
    1, 0, 0 ,0 ,0 ,0
    2, 0, 3, 1, 1, 0]
    */
    fn generate_tile_data(&self, tileset_dimensions: &TilesetDimensions) -> Result<TilemapTiles, TileDataError> {
        let &TilesetDimensions{ width, height } = tileset_dimensions;
        self.layers().iter()
            .flat_map(|layer| layer.tiles().iter()
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
}

impl TiledMap< ImageSource, tiled::Image, tiled::Tileset, tiled::Layer> for tiled::Map {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn tilesets(&self) -> &Vec<tiled::Tileset> {
        &self.tilesets
    }

    fn layers(&self) -> &Vec<tiled::Layer> {
        &self.layers
    }
}

trait TiledLayer {
    fn tiles(&self) -> &Vec<Vec<u32>>;
}

impl TiledLayer for tiled::Layer {
    fn tiles(&self) -> &Vec<Vec<u32>> {
        &self.tiles
    }
}

trait TiledTileset<TImageSource, TImage: TiledImage<TImageSource>> {
    fn tile_width(&self) -> u32;
    fn tile_height(&self) -> u32;
    fn images(&self) -> &Vec<TImage>;
    fn fetch_tileset_image(&self) -> Result<&TImage, FetchImageError>;
    fn fetch_tileset_dims(&self, img: &TImage) -> Result<TilesetDimensions, Box<Error>> {
        let tileset_width = CheckedDiv::checked_div(&(img.width() as u32), &self.tile_width())
            .ok_or_else(|| FetchTilesetWidthError)?;
        let tileset_height = CheckedDiv::checked_div(&(img.height() as u32), &self.tile_height())
            .ok_or_else(|| FetchTilesetHeightError)?;

        Ok(TilesetDimensions {
            width: tileset_width,
            height: tileset_height
        })
    }
}

impl TiledTileset<ImageSource, tiled::Image> for tiled::Tileset {
    fn tile_width(&self) -> u32 {
        self.tile_width
    }

    fn tile_height(&self) -> u32 {
        self.tile_height
    }

    fn images(&self) -> &Vec<tiled::Image> {
        &self.images
    }

    fn fetch_tileset_image(&self) -> Result<&tiled::Image, FetchImageError> {
        self.images().get(0).ok_or_else(|| FetchImageError)
    }
}

trait TiledImage<TImageSource> {
    // Yes, for some reason, tiled uses i32 for width and height. Because widths can be negative, right? :)
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn fetch_img_src(&self) -> TImageSource;
}

impl TiledImage<ImageSource> for tiled::Image {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn fetch_img_src(&self) -> ImageSource {
        ImageSource(format!("{}{}", "../resources/", self.source))
    }
}

struct ImageSource(String);

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

    /*
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
*/
    #[test]
    fn tilemap_info_test() {
        /*let mock = MockTiledMap::<u32, u32> {
            width: 64,
            height: 64,
            layers: Vec::new(),
            tilesets: Vec::new()
        };*/
        assert_eq!(true, true)
    }
}