src/
├── main.rs                 # Binary entry point
├── lib.rs                  # Library exports (if needed)
├── config/
│   ├── mod.rs             # Re-export config types
│   ├── settings.rs        # Application settings
│   └── http.rs            # HTTP-specific configuration
├── api/                   # Renamed from handlers/ for clarity
│   ├── mod.rs
│   ├── routes.rs          # Route definitions
│   ├── health.rs
│   ├── ifsc.rs
│   └── stats.rs
├── services/
│   ├── mod.rs
│   ├── ifsc/
│   │   ├── mod.rs
│   │   ├── service.rs     # IFSCService implementation
│   │   ├── client.rs      # HTTP client for external APIs
│   │   └── cache.rs       # IFSC-specific caching
│   └── validation/
│       ├── mod.rs
│       └── service.rs     # Validation logic
├── storage/
│   ├── mod.rs
│   ├── database/
│   │   ├── mod.rs
│   │   ├── postgres.rs    # Database implementations
│   │   └── models.rs      # Database-specific models
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── redis.rs
│   │   └── memory.rs
│   └── repositories/      # Data access layer
│       ├── mod.rs
│       ├── bank_repo.rs
│       └── ifsc_repo.rs
├── models/
│   ├── mod.rs
│   ├── api/               # API models (request/response)
│   │   ├── mod.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── domain/            # Domain models (business logic)
│   │   ├── mod.rs
│   │   ├── bank.rs
│   │   └── ifsc.rs
│   └── errors.rs
├── middleware/
│   ├── mod.rs
│   ├── logging.rs
│   ├── auth.rs
│   └── rate_limit.rs
├── utils/
│   ├── mod.rs
│   ├── validation.rs
│   ├── logging.rs
│   └── macros.rs          # Custom macros if needed
└── telemetry/             # Observability (important!)
├── mod.rs
├── metrics.rs
├── tracing.rs
└── health_check.rs


tests/
├── api/
│   ├── health_test.rs
│   └── ifsc_test.rs
├── services/
│   ├── ifsc_service_test.rs
│   └── validation_test.rs
├── storage/
│   └── repository_test.rs
└── integration/
└── app_test.rs