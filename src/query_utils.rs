use sqlx::{QueryBuilder, Sqlite};

pub fn add_in_expression<'a, T>(builder: &mut QueryBuilder<'a, Sqlite>, elements: &'a [T])
where
    T: sqlx::Encode<'a, Sqlite> + sqlx::Type<Sqlite> + Send + Sync,
{
    if elements.is_empty() {
        return;
    }

    builder.push(" IN (");
    for (i, element) in elements.iter().enumerate() {
        builder.push_bind(element);
        if i < elements.len() - 1 {
            builder.push(",");
        }
    }

    builder.push(") ");
}
