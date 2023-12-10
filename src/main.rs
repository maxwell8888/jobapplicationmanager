use ravascript::web::*;

fn query_selector(id: &str) -> MyNode {
    MyNode::new(Document::query_selector(id))
}
fn create_element(tag: &str) -> MyNode {
    MyNode::new(Document::create_element(tag))
}
fn create_text_node(text: &str) -> MyNode {
    MyNode::new(Document::create_text_node(text))
}

struct MyNode {
    node: DomNode,
}
impl MyNode {
    fn new(node: DomNode) -> Self {
        Self { node }
    }

    fn class(self, class_names: &'static str) -> Self {
        self.node.set_attribute("class", class_names);
        self
    }
    fn with_text_content(mut self, text: &'static str) -> Self {
        self.node.text_content = text;
        self
    }
    fn class_list_add(self, class_name: &str) -> Self {
        self.node.class_list.add(class_name);
        self
    }
    fn with_attr(self, attr_name: &str, attr_val: &str) -> Self {
        self.node.set_attribute(attr_name, attr_val);
        self
    }
    fn with_child(self, my_node: MyNode) -> Self {
        self.node.append_child(my_node.node);
        self
    }
    fn with_children(self, my_nodes: Vec<MyNode>) -> Self {
        for node in my_nodes {
            self.node.append_child(node.node);
        }
        self
    }
}

fn expanding_textarea() -> MyNode {
    create_element("div").class("grid after::content[content: attr(data-replicated-value) ' '] after::whitespace-pre-wrap after::invisible after::border-2 after::border-black after::p-2 after::row-start-1 after::row-end-2 after::col-start-1 after::col-end-2").with_child(create_element("textarea").class("resize-none overflow-hidden border-2 border-black p-2 row-start-1 row-end-2 col-start-1 col-end-2").with_attr("onInput", "this.parentNode.dataset.replicatedValue = this.value"))
}

fn get_body() -> Body {
    Document::query_selector2::<Body>("body")
}
pub struct Div {
    dom_node: DomNode,
}
impl Div {
    fn new() -> Div {
        let dom_node = Document::create_element("div");
        Div { dom_node }
    }
    // make() {
    //     const domNode = document.createElement("div");
    //     return new Div(domNode)
    // }
    fn get_self2(self) -> Self {
        self
    }
}
// impl DomNodeTrait for Div {}
impl Element for Div {
    fn append_child<T: DomNodeTrait>(&self, child: T) {
        self.dom_node.append_child2(child);
    }
    fn get_self() -> Self {
        Div {
            dom_node: Document::create_element("div"),
        }
    }
}

fn main() {
    let body = get_body();
    let div = Div::new();
    let text = Document::create_text_node("hello");
    div.append_child(text);
    body.append_child(div.dom_node);
}
