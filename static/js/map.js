let map;
let directionsService;
let directionsRenderer;
let placesService;
let autocompleteStart;
let autocompleteEnd;
let markers = [];
let currentLocationMarker;

function initMap() {
    // Initialize map centered on San Francisco
    map = new google.maps.Map(document.getElementById('map'), {
        zoom: 13,
        center: { lat: 37.7749, lng: -122.4194 },
        mapTypeControl: true,
        streetViewControl: true,
        fullscreenControl: true,
        zoomControl: true
    });

    // Initialize directions service and renderer
    directionsService = new google.maps.DirectionsService();
    directionsRenderer = new google.maps.DirectionsRenderer({
        draggable: true,
        panel: document.getElementById('directions-panel')
    });
    directionsRenderer.setMap(map);

    // Initialize places service
    placesService = new google.maps.places.PlacesService(map);

    // Set up autocomplete for search inputs
    setupAutocomplete();

    // Get user's current location
    getCurrentLocation();

    // Set up event listeners
    setupEventListeners();

    // Handle directions renderer drag events
    directionsRenderer.addListener('directions_changed', function() {
        const directions = directionsRenderer.getDirections();
        updateRouteInfo(directions);
    });
}

function setupAutocomplete() {
    const startInput = document.getElementById('start-location');
    const endInput = document.getElementById('end-location');

    if (startInput) {
        autocompleteStart = new google.maps.places.Autocomplete(startInput);
        autocompleteStart.bindTo('bounds', map);
        autocompleteStart.addListener('place_changed', function() {
            const place = autocompleteStart.getPlace();
            if (place.geometry) {
                map.panTo(place.geometry.location);
                addMarker(place.geometry.location, place.name, 'start');
            }
        });
    }

    if (endInput) {
        autocompleteEnd = new google.maps.places.Autocomplete(endInput);
        autocompleteEnd.bindTo('bounds', map);
        autocompleteEnd.addListener('place_changed', function() {
            const place = autocompleteEnd.getPlace();
            if (place.geometry) {
                addMarker(place.geometry.location, place.name, 'end');
            }
        });
    }
}

function setupEventListeners() {
    // Search button
    const searchBtn = document.getElementById('search-btn');
    if (searchBtn) {
        searchBtn.addEventListener('click', performSearch);
    }

    // Directions button
    const directionsBtn = document.getElementById('directions-btn');
    if (directionsBtn) {
        directionsBtn.addEventListener('click', calculateDirections);
    }

    // Clear button
    const clearBtn = document.getElementById('clear-btn');
    if (clearBtn) {
        clearBtn.addEventListener('click', clearMap);
    }

    // My location button
    const myLocationBtn = document.getElementById('my-location-btn');
    if (myLocationBtn) {
        myLocationBtn.addEventListener('click', getCurrentLocation);
    }

    // Map type selector
    const mapTypeSelect = document.getElementById('map-type');
    if (mapTypeSelect) {
        mapTypeSelect.addEventListener('change', function() {
            map.setMapTypeId(this.value);
        });
    }

    // Traffic toggle
    const trafficToggle = document.getElementById('traffic-toggle');
    if (trafficToggle) {
        const trafficLayer = new google.maps.TrafficLayer();
        trafficToggle.addEventListener('change', function() {
            if (this.checked) {
                trafficLayer.setMap(map);
            } else {
                trafficLayer.setMap(null);
            }
        });
    }

    // Map click event for adding markers
    map.addListener('click', function(event) {
        if (event.placeId) {
            event.stop();
            getPlaceDetails(event.placeId, event.latLng);