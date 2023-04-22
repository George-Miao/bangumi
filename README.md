# Bangumi

 This is a Rust library for the Bangumi.moe API based on [rustified](https://github.com/George-Miao/rustified).

## Usage

 ```rust
 # #[tokio::test] async fn doc_test_1() -> Result<(), Box<dyn std::error::Error>>{ use bangumi::*;
 use bangumi::{endpoints::GetCurrent, Endpoint};

 let client = bangumi::client();
 let result: Vec<WithId<Bangumi>> = GetCurrent.exec(&client).await?.parse()?;
 # Ok(()) }
```

## With builder

 ```rust
 # #[tokio::test] async fn doc_test_2() -> Result<(), Box<dyn std::error::Error>>{ use bangumi::*;
 use bangumi::{endpoints::SearchTags, Endpoint};

 let client = bangumi::client();
 let result: SearchResult<Vec<WithId<Tag>>> =
     SearchTags::builder()
         .name("魔法少女")
         .keywords(false)
         .tag_type(TagType::Bangumi)
         .build()
         .exec(&client)
         .await?
         .parse()?;
 # Ok(()) }
```

 For all endpoints, see [endpoints](endpoints/index.html).
