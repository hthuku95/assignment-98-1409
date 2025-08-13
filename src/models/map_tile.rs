use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::geo::{LatLng, BoundingBox};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MapTile {
    pub x: u32,
    pub y: u32,
    pub z: u8,
    pub data: Vec<u8>,
    pub format: TileFormat,
    pub timestamp: u64,
    pub size: usize,
    pub metadata: TileMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TileFormat {
    Png,
    Jpeg,
    Webp,
    Vector,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TileMetadata {
    pub bounds: BoundingBox,
    pub features: Vec<MapFeature>,
    pub roads: Vec<Road>,
    pub labels: Vec<Label>,
    pub pois: Vec<PointOfInterest>,
    pub style_version: String,
    pub data_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MapFeature {
    pub id: String,
    pub feature_type: FeatureType,
    pub geometry: Geometry,
    pub properties: HashMap<String, String>,
    pub style: FeatureStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeatureType {
    Building,
    Water,
    Park,
    Forest,
    Administrative,
    Landuse,
    Natural,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Geometry {
    Point(LatLng),
    LineString(Vec<LatLng>),
    Polygon(Vec<Vec<LatLng>>),
    MultiPolygon(Vec<Vec<Vec<LatLng>>>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureStyle {
    pub fill_color: Option<String>,
    pub stroke_color: Option<String>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
    pub z_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Road {
    pub id: String,
    pub name: String,
    pub road_type: RoadType,
    pub geometry: Vec<LatLng>,
    pub lanes: u8,
    pub speed_limit: Option<u32>,
    pub one_way: bool,
    pub surface: RoadSurface,
    pub style: RoadStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoadType {
    Highway,
    Primary,
    Secondary,
    Tertiary,
    Residential,
    Service,
    Footway,
    Cycleway,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoadSurface {
    Paved,
    Unpaved,
    Gravel,
    Dirt,
    Grass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoadStyle {
    pub color: String,
    pub width: f32,
    pub dash_pattern: Option<Vec<f32>>,
    pub border_color: Option<String>,
    pub border_width: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Label {
    pub id: String,
    pub text: String,
    pub position: LatLng,
    pub label_type: LabelType,
    pub font_size: f32,
    pub font_family: String,
    pub color: String