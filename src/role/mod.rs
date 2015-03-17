use gli;

pub trait Generator: gli::Generate {
    type Object: gli::IntoObject<Self::Object>;

    fn gen_one(&self) -> Self::Object {
        debug!("gen, size = one");

        let id = <Self as gli::Generate>::gl_gen(1)[0];

        debug!("[{}]: generated", id);

        <Self::Object as gli::IntoObject<Self::Object>>::new_object(id)
    }

    fn gen(&self, size: usize) -> Vec<Self::Object> {
        debug!("gen, size = {}", size);

        let ids = <Self as gli::Generate>::gl_gen(size);

        debug!(
            "[{}]: generated",
            ids.iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .connect(", ")
        );

        ids
            .into_iter()
            .map(|id| <Self::Object as gli::IntoObject<Self::Object>>::new_object(id) )
            .collect()
    }
}
