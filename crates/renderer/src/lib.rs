trait Api {
    type Renderer: Renderer<Self>;
    type Pipeline: Pipeline;
}


trait Renderer<A: Api> {
    fn add_pipeline() -> A::Pipeline;
    fn remove_pipeline(pipeline: &mut A::Pipeline);
}

trait Pipeline {

}
