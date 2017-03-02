//! Command & Query repository traits definition.

/// Repository trait used to query a persisted entity.
///
/// Use this trait to implement the `get` and `get_all` methods for a given entity `T` having the
/// type `U` as its primary key type.
pub trait QueryRepository<T, U> {

    /// Get a single `T` entity using its `U` primary key.
    fn get(&self, pk: U) -> Option<T>;

    /// Get all the persisted `T` entities as a `Vec<T>`.
    fn get_all(&self) -> Option<Vec<T>>;
}

/// Repository trait used to command a persisted entity.
///
/// Use this trait to implement the bare command methods for a given entity `T` having the type `U`
/// as its primary key type.
pub trait CommandRepository<T, U> {

    /// Persist a new `T` entity and return its generated `U` primary key in a `Result` object.
    fn save(&self, entity: T) -> Result<U, String>;

    /// Update all the fields of an already persisted `T` entity and return the updated entity
    /// wrapped in a `Result` object.
    fn update(&self, entity: T) -> Result<T, String>;

    /// Remove the specified `T` entity from the persistence context and returns `true` if everything
    /// went right, or an error as a `String`.
    fn delete(&self, entity: T) -> Result<bool, String>;
}