use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_active -> Bool,
    }
}

table! {
    locations (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        latitude -> Float8,
        longitude -> Float8,
        address -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        postal_code -> Nullable<Varchar>,
        place_type -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        created_by -> Nullable<Uuid>,
    }
}

table! {
    routes (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        start_location_id -> Uuid,
        end_location_id -> Uuid,
        distance_meters -> Int8,
        duration_seconds -> Int8,
        geometry -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        created_by -> Nullable<Uuid>,
    }
}

table! {
    route_steps (id) {
        id -> Uuid,
        route_id -> Uuid,
        step_order -> Int4,
        instruction -> Text,
        distance_meters -> Int8,
        duration_seconds -> Int8,
        start_latitude -> Float8,
        start_longitude -> Float8,
        end_latitude -> Float8,
        end_longitude -> Float8,
        maneuver -> Nullable<Varchar>,
    }
}

table! {
    favorites (id) {
        id -> Uuid,
        user_id -> Uuid,
        location_id -> Uuid,
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

table! {
    search_history (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        query -> Varchar,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        results_count -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    traffic_data (id) {
        id -> Uuid,
        location_id -> Uuid,
        traffic_level -> Int4,
        speed_kmh -> Nullable<Float8>,
        congestion_factor -> Float8,
        timestamp -> Timestamptz,
        source -> Varchar,
    }
}

table! {
    poi_categories (id) {
        id -> Uuid,
        name -> Varchar,
        icon -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

table! {
    points_of_interest (id) {
        id -> Uuid,
        location_id -> Uuid,
        category_id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        rating -> Nullable<Float8>,
        price_level -> Nullable<Int4>,
        phone -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        opening_hours -> Nullable<Text>,
        photos -> Nullable<Array<Text>>,
        verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    reviews (id) {
        id -> Uuid,
        poi_id -> Uuid,
        user_id -> Nullable<Uuid>,
        rating -> Int4,
        comment -> Nullable<Text>,
        photos -> Nullable<Array<Text>>,
        helpful_count -> Int4,
        created_at -> Timestamptz,