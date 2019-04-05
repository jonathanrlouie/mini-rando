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
    TImageSource,
    TImage: TiledImage<TImageSource>,
    TTileset: TiledTileset<TImageSource, TImage>,
    TLayer: TiledLayer,
    TMap: TiledMap< TImageSource, TImage, TTileset, TLayer>
> {
    fn create_map(&self) -> Result<TMap, Box<Error>>;

    fn initialise_tilemap(&self) -> Result<TilemapInfo<TImageSource>, Box<Error>> {
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

impl<'a> TilemapCreator<ImageSource, tiled::Image, tiled::Tileset, tiled::Layer, tiled::Map> for TilemapPath<'a> {
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

    fn fetch_tileset(&self) -> Result<&TTileset, FetchTilesetError> {
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

    fn fetch_tileset_image(&self) -> Result<&TImage, FetchImageError> {
        self.images().get(0).ok_or_else(|| FetchImageError)
    }

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

    struct MockMap {
        width: u32,
        height: u32,
        layers: Vec<MockLayer>,
        tilesets: Vec<MockTileset>
    }

    impl TiledMap<ImageSource, MockImage, MockTileset, MockLayer> for MockMap {
        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn tilesets(&self) -> &Vec<MockTileset> {
            &self.tilesets
        }

        fn layers(&self) -> &Vec<MockLayer> {
            &self.layers
        }
    }

    struct MockLayer {
        tiles: Vec<Vec<u32>>
    }

    impl TiledLayer for MockLayer {
        fn tiles(&self) -> &Vec<Vec<u32>> {
            &self.tiles
        }
    }

    struct MockTileset {
        tile_width: u32,
        tile_height: u32,
        images: Vec<MockImage>
    }

    impl TiledTileset<ImageSource, MockImage> for MockTileset {
        fn tile_width(&self) -> u32 {
            self.tile_width
        }

        fn tile_height(&self) -> u32 {
            self.tile_height
        }

        fn images(&self) -> &Vec<MockImage> {
            &self.images
        }
    }

    struct MockImage {
        width: i32,
        height: i32
    }

    impl TiledImage<ImageSource> for MockImage {
        fn width(&self) -> i32 {
            self.width
        }

        fn height(&self) -> i32 {
            self.height
        }

        fn fetch_img_src(&self) -> ImageSource {
            ImageSource("".to_string())
        }
    }

    struct MockTilemapPath {
        mock_map: fn() -> MockMap
    }

    impl TilemapCreator<ImageSource, MockImage, MockTileset, MockLayer, MockMap> for MockTilemapPath {
        fn create_map(&self) -> Result<MockMap, Box<Error>> {
            Ok((self.mock_map)())
        }
    }

    #[test]
    fn tilemap_info_test() {
        // number of tiles on x-axis: 20
        // number of tiles on y-axis: 15
        // tileset looks something like this:
        // |- - -| * * * * * * * * * * ...
        // |     | * * * * * * * * * * ...
        // |- - -| * * * * * * * * * * ...
        // ...
        //
        // Origin of final coords is lower left corner of the tileset image.
        // Tilemap indices have 1 subtracted from them (so 0 is a blank square).
        // Indexing of tilemap tiles starts from top-left corner of tileset.
        // As a result, 6 becomes 5, which causes the final coord to be (5, 14).
        let mock_map = || MockMap {
            width: 5,
            height: 5,
            layers: vec![
                MockLayer {
                    tiles: vec![
                        vec![0,  0,  0,  0, 0],
                        vec![0,  1,  2,  3, 0],
                        vec![0, 21, 22, 23, 0],
                        vec![0, 41, 42, 43, 0],
                        vec![0,  0,  0,  0, 0],
                    ]
                }
            ],
            tilesets: vec![
                MockTileset {
                    tile_width: 32,
                    tile_height: 32,
                    images: vec![
                        MockImage {
                            width: 640,
                            height: 480
                        }
                    ]
                }
            ]
        };
        let mock = MockTilemapPath {
            mock_map
        };
        let tilemap_info = mock.initialise_tilemap().expect("Error creating tile map info.");
        let tiles: Vec<[f32; 4]> = tilemap_info.tilemap_tiles.tiles;

        // first row
        assert_eq!(tiles[0][0] as u32, 0);
        assert_eq!(tiles[0][1] as u32, 0);

        assert_eq!(tiles[1][0] as u32, 0);
        assert_eq!(tiles[1][1] as u32, 0);

        assert_eq!(tiles[2][0] as u32, 0);
        assert_eq!(tiles[2][1] as u32, 0);

        assert_eq!(tiles[3][0] as u32, 0);
        assert_eq!(tiles[3][1] as u32, 0);

        assert_eq!(tiles[4][0] as u32, 0);
        assert_eq!(tiles[4][1] as u32, 0);

        // second row
        assert_eq!(tiles[5][0] as u32, 0);
        assert_eq!(tiles[5][1] as u32, 0);

        assert_eq!(tiles[6][0] as u32, 0);
        assert_eq!(tiles[6][1] as u32, 14);

        assert_eq!(tiles[7][0] as u32, 1);
        assert_eq!(tiles[7][1] as u32, 14);

        assert_eq!(tiles[8][0] as u32, 2);
        assert_eq!(tiles[8][1] as u32, 14);

        assert_eq!(tiles[9][0] as u32, 0);
        assert_eq!(tiles[9][1] as u32, 0);

        // third row
        assert_eq!(tiles[10][0] as u32, 0);
        assert_eq!(tiles[10][1] as u32, 0);

        assert_eq!(tiles[11][0] as u32, 0);
        assert_eq!(tiles[11][1] as u32, 13);

        assert_eq!(tiles[12][0] as u32, 1);
        assert_eq!(tiles[12][1] as u32, 13);

        assert_eq!(tiles[13][0] as u32, 2);
        assert_eq!(tiles[13][1] as u32, 13);

        assert_eq!(tiles[14][0] as u32, 0);
        assert_eq!(tiles[14][1] as u32, 0);

        // fourth row
        assert_eq!(tiles[15][0] as u32, 0);
        assert_eq!(tiles[15][1] as u32, 0);

        assert_eq!(tiles[16][0] as u32, 0);
        assert_eq!(tiles[16][1] as u32, 12);

        assert_eq!(tiles[17][0] as u32, 1);
        assert_eq!(tiles[17][1] as u32, 12);

        assert_eq!(tiles[18][0] as u32, 2);
        assert_eq!(tiles[18][1] as u32, 12);

        assert_eq!(tiles[19][0] as u32, 0);
        assert_eq!(tiles[19][1] as u32, 0);

        // fifth row
        assert_eq!(tiles[20][0] as u32, 0);
        assert_eq!(tiles[20][1] as u32, 0);

        assert_eq!(tiles[21][0] as u32, 0);
        assert_eq!(tiles[21][1] as u32, 0);

        assert_eq!(tiles[22][0] as u32, 0);
        assert_eq!(tiles[22][1] as u32, 0);

        assert_eq!(tiles[23][0] as u32, 0);
        assert_eq!(tiles[23][1] as u32, 0);

        assert_eq!(tiles[24][0] as u32, 0);
        assert_eq!(tiles[24][1] as u32, 0);
    }
}