//! Types for all standard HTML5 elements.

use typed_html_macros::declare_elements;

use dom::{Node, TextNode};
use types::*;

// Marker traits for element content groups

pub trait MetadataContent: Node {}
pub trait FlowContent: Node {}
pub trait SectioningContent: Node {}
pub trait HeadingContent: Node {}
// Phrasing content seems to be entirely a subclass of FlowContent
pub trait PhrasingContent: FlowContent {}
pub trait EmbeddedContent: Node {}
pub trait InteractiveContent: Node {}
pub trait FormContent: Node {}

// Traits for elements that are more picky about their children
pub trait DescriptionListContent: Node {}
pub trait HGroupContent: Node {}
pub trait MapContent: Node {}
pub trait MediaContent: Node {} // <audio> and <video>
pub trait SelectContent: Node {}
pub trait TableContent: Node {}
pub trait TableColumnContent: Node {}

declare_elements!{
    html {
        xmlns: Uri,
    } with [head, body];
    head with [title] MetadataContent;
    body with FlowContent;

    // Metadata
    base {
        href: Uri,
        target: Target,
    } in [MetadataContent];
    link {
        as: Mime,
        crossorigin: CrossOrigin,
        href: Uri,
        hreflang: LanguageTag,
        media: String, // FIXME media query
        rel: LinkType,
        sizes: String, // FIXME
        title: String, // FIXME
        type: Mime,
    } in [MetadataContent];
    meta {
        charset: String, // FIXME IANA standard names
        content: String,
        http_equiv: String, // FIXME string enum
        name: String, // FIXME string enum
    } in [MetadataContent];
    style {
        type: Mime,
        media: String, // FIXME media query
        nonce: Nonce,
        title: String, // FIXME
    } in [MetadataContent] with TextNode;
    title in [MetadataContent] with TextNode;

    // Flow
    a {
        download: String,
        href: Uri,
        hreflang: LanguageTag,
        ping: SpacedList<Uri>,
        rel: SpacedList<LinkType>,
        target: Target,
        type: Mime,
    } in [FlowContent, PhrasingContent, InteractiveContent] with FlowContent;
    abbr in [FlowContent, PhrasingContent] with PhrasingContent;
    address in [FlowContent] with FlowContent;
    article in [FlowContent, SectioningContent] with FlowContent;
    aside in [FlowContent, SectioningContent] with FlowContent;
    audio {
        autoplay: bool,
        controls: bool,
        crossorigin: CrossOrigin,
        loop: bool,
        muted: bool,
        preload: Preload,
        src: Uri,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with MediaContent;
    b in [FlowContent, PhrasingContent] with PhrasingContent;
    bdo in [FlowContent, PhrasingContent] with PhrasingContent;
    bdi in [FlowContent, PhrasingContent] with PhrasingContent;
    blockquote {
        cite: Uri,
    } in [FlowContent] with FlowContent;
    br in [FlowContent, PhrasingContent];
    button {
        autofocus: bool,
        disabled: bool,
        form: Id,
        formaction: Uri,
        formenctype: FormEncodingType,
        formmethod: FormMethod,
        formnovalidate: bool,
        formtarget: Target,
        name: Id,
        type: ButtonType,
        value: String,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with PhrasingContent;
    canvas {
        height: usize,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with FlowContent;
    cite in [FlowContent, PhrasingContent] with PhrasingContent;
    code in [FlowContent, PhrasingContent] with PhrasingContent;
    data {
        value: String,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    datalist in [FlowContent, PhrasingContent] with option;
    del {
        cite: Uri,
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with FlowContent;
    details {
        open: bool,
    } in [FlowContent, SectioningContent, InteractiveContent] with [summary] FlowContent;
    dfn in [FlowContent, PhrasingContent] with PhrasingContent;
    div in [FlowContent] with FlowContent;
    dl in [FlowContent] with DescriptionListContent;
    em in [FlowContent, PhrasingContent] with PhrasingContent;
    embed {
        height: usize,
        src: Uri,
        type: Mime,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent];
    // FIXME the legend attribute should be optional
    fieldset in [FlowContent, SectioningContent, FormContent] with [legend] FlowContent;
    // FIXME the figcaption attribute should be optional
    figure in [FlowContent, SectioningContent] with [figcaption] FlowContent;
    footer in [FlowContent] with FlowContent;
    form {
        accept-charset: SpacedList<CharacterEncoding>,
        action: Uri,
        autocomplete: OnOff,
        enctype: FormEncodingType,
        method: FormMethod,
        name: Id,
        novalidate: bool,
        target: Target,
    } in [FlowContent] with FlowContent;
    h1 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h2 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h3 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h4 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h5 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h6 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    header in [FlowContent] with FlowContent;
    hgroup in [FlowContent, HeadingContent] with HGroupContent;
    hr in [FlowContent];
    i in [FlowContent, PhrasingContent] with PhrasingContent;
    iframe {
        allow: FeaturePolicy,
        allowfullscreen: bool,
        allowpaymentrequest: bool,
        height: usize,
        name: Id,
        referrerpolicy: ReferrerPolicy,
        sandbox: SpacedSet<Sandbox>,
        src: Uri,
        srcdoc: Uri,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent] with FlowContent;
    img {
        alt: String,
        crossorigin: CrossOrigin,
        decoding: ImageDecoding,
        height: usize,
        ismap: bool,
        sizes: SpacedList<String>, // FIXME it's not really just a string
        src: Uri,
        srcset: String, // FIXME this is much more complicated
        usemap: String, // FIXME should be a fragment starting with '#'
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent];
    input {
        autocomplete: String,
        autofocus: bool,
        disabled: bool,
        form: Id,
        list: Id,
        name: Id,
        required: bool,
        tabindex: usize,
        type: InputType,
        value: String,
    } in [FlowContent, FormContent, PhrasingContent];
    ins {
        cite: Uri,
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with FlowContent;
    kbd in [FlowContent, PhrasingContent] with PhrasingContent;
    label {
        for: Id,
        form: Id,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with PhrasingContent;
    main in [FlowContent] with FlowContent;
    map {
        name: Id,
    } in [FlowContent, PhrasingContent] with MapContent;
    mark in [FlowContent, PhrasingContent] with PhrasingContent;
    // TODO the <math> element
    meter {
        value: isize,
        min: isize,
        max: isize,
        low: isize,
        high: isize,
        optimum: isize,
        form: Id,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    nav in [FlowContent, SectioningContent] with PhrasingContent;
    noscript in [MetadataContent, FlowContent, PhrasingContent] with Node;
    object {
        data: Uri,
        form: Id,
        height: usize,
        name: Id,
        type: Mime,
        typemustmatch: bool,
        usemap: String, // TODO should be a fragment starting with '#'
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent, FormContent] with param;
    ol {
        reversed: bool,
        start: isize,
        type: OrderedListType,
    } in [FlowContent] with li;
    output {
        for: SpacedSet<Id>,
        form: Id,
        name: Id,
    } in [FlowContent, PhrasingContent, FormContent] with PhrasingContent;
    p in [FlowContent] with PhrasingContent;
    pre in [FlowContent] with PhrasingContent;
    progress {
        max: f64,
        value: f64,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    q {
        cite: Uri,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    ruby in [FlowContent, PhrasingContent] with PhrasingContent;
    s in [FlowContent, PhrasingContent] with PhrasingContent;
    samp in [FlowContent, PhrasingContent] with PhrasingContent;
    script {
        async: bool,
        crossorigin: CrossOrigin,
        defer: bool,
        integrity: Integrity,
        nomodule: bool,
        nonce: Nonce,
        src: Uri,
        text: String,
        type: String, // TODO could be an enum
    } in [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] with TextNode;
    section in [FlowContent, SectioningContent] with FlowContent;
    select {
        autocomplete: String,
        autofocus: bool,
        disabled: bool,
        form: Id,
        multiple: bool,
        name: Id,
        required: bool,
        size: usize,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with SelectContent;
    small in [FlowContent, PhrasingContent] with PhrasingContent;
    span in [FlowContent, PhrasingContent] with PhrasingContent;
    strong in [FlowContent, PhrasingContent] with PhrasingContent;
    sub in [FlowContent, PhrasingContent] with PhrasingContent;
    sup in [FlowContent, PhrasingContent] with PhrasingContent;
    table in [FlowContent] with TableContent;
    template in [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] with Node;
    textarea {
        autocomplete: OnOff,
        autofocus: bool,
        cols: usize,
        disabled: bool,
        form: Id,
        maxlength: usize,
        minlength: usize,
        name: Id,
        placeholder: String,
        readonly: bool,
        required: bool,
        rows: usize,
        spellcheck: BoolOrDefault,
        wrap: Wrap,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with TextNode;
    time {
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    ul in [FlowContent] with li;
    var in [FlowContent, PhrasingContent] with PhrasingContent;
    video in [FlowContent, PhrasingContent, EmbeddedContent] with MediaContent;
    wbr in [FlowContent, PhrasingContent];

    // Non-group elements
    area {
        alt: String,
        coords: String, // TODO could perhaps be validated
        download: bool,
        href: Uri,
        hreflang: LanguageTag,
        ping: SpacedList<Uri>,
        rel: SpacedSet<LinkType>,
        shape: AreaShape,
        target: Target,
    } in [MapContent];
    caption in [TableContent] with FlowContent;
    col {
        span: usize,
    };
    colgroup {
        span: usize,
    } in [TableContent] with col;
    dd in [DescriptionListContent] with FlowContent;
    dt in [DescriptionListContent] with FlowContent;
    figcaption with FlowContent;
    legend with PhrasingContent;
    li {
        value: isize,
    } with FlowContent;
    option {
        disabled: bool,
        label: String,
        selected: bool,
        value: String,
    } in [SelectContent] with TextNode;
    optgroup {
        disabled: bool,
        label: String,
    } in [SelectContent] with option;
    param {
        name: String,
        value: String,
    };
    source {
        src: Uri,
        type: Mime,
    } in [MediaContent];
    summary with PhrasingContent;
    tbody in [TableContent] with tr;
    td {
        colspan: usize,
        headers: SpacedSet<Id>,
        rowspan: usize,
    } in [TableColumnContent] with FlowContent;
    tfoot in [TableContent] with tr;
    th {
        abbr: String,
        colspan: usize,
        headers: SpacedSet<Id>,
        rowspan: usize,
        scope: TableHeaderScope,
    } in [TableColumnContent] with FlowContent;
    thead in [TableContent] with tr;
    tr in [TableContent] with TableColumnContent;
    track {
        default: bool,
        kind: VideoKind,
        label: String,
        src: Uri,
        srclang: LanguageTag,
    } in [MediaContent];

    // Don't @ me
    blink in [FlowContent, PhrasingContent] with PhrasingContent;
    marquee {
        behavior: String, // FIXME enum
        bgcolor: String, // FIXME colour
        direction: String, // FIXME direction enum
        height: String, // FIXME size
        hspace: String, // FIXME size
        loop: isize,
        scrollamount: usize,
        scrolldelay: usize,
        truespeed: bool,
        vspace: String, // FIXME size
        width: String, // FIXME size
    } in [FlowContent, PhrasingContent] with PhrasingContent;
}
