pub mod new {

    pub fn user_script(
        css: String,
        theme_name: String,
        namespace: String,
        version: String,
        description: String,
        author: String,
        homepage_url: String,
        support_url: String,
        update_url: String,
        url_regex: String,
    ) -> String {
        let mut modified_css = "".to_string();
        for line in css.split("\n") {
            modified_css = modified_css
                + "\t\t\""
                + &line.replace("\"", "\\\"").to_string().trim()
                + "\","
                + "\n";
        }
        return userscript_template()
            .populate_meta(
                theme_name,
                namespace,
                version,
                description,
                author,
                homepage_url,
                support_url,
                update_url,
                url_regex,
            )
            .replace("<Style>", &modified_css);
    }
    pub fn stylus(
        css: String,
        theme_name: String,
        namespace: String,
        version: String,
        description: String,
        author: String,
        homepage_url: String,
        support_url: String,
        update_url: String,
        url_regex: String,
    ) -> String {
        return stylus_template()
            .populate_meta(
                theme_name,
                namespace,
                version,
                description,
                author,
                homepage_url,
                support_url,
                update_url,
                url_regex,
            )
            .replace("<Style>", &css);
    }

    trait Helpers {
        fn populate_meta(
            &self,
            theme_name: String,
            namespace: String,
            version: String,
            description: String,
            author: String,
            homepage_url: String,
            support_url: String,
            update_url: String,
            url_regex: String,
        ) -> String;
    }

    impl Helpers for String {
        fn populate_meta(
            &self,
            theme_name: String,
            namespace: String,
            version: String,
            description: String,
            author: String,
            homepage_url: String,
            support_url: String,
            update_url: String,
            url_regex: String,
        ) -> String {
            return self
                .replace("<ThemeName>", &theme_name)
                .replace("<Namespace>", &namespace)
                .replace("<Version>", &version)
                .replace("<Description>", &description)
                .replace("<Author>", &author)
                .replace("<HomepageUrl>", &homepage_url)
                .replace("<SupportUrl>", &support_url)
                .replace("<UpdateUrl>", &update_url)
                .replace("<Url_Regex>", &url_regex);
        }
    }

    fn stylus_template() -> String {
        return r##"/* ==UserStyle==
@name          <ThemeName>
@namespace     <Namespace>
@version       <Version>
@description   <Description>
@author        <Author>
@homepageURL   <HomepageUrl>
@supportURL    <SupportUrl>
@updateURL     <UpdateUrl>
==/UserStyle== */

@-moz-document regexp("<Url_Regex>") {
<Style>
}"##
        .to_string();
    }

    fn userscript_template() -> String {
        return r##"// ==UserScript==
// @name        <ThemeName>
// @namespace   <Namespace>
// @version     <Version>
// @description <Description>
// @author      <Author>
// @homepage    <HomepageUrl>
// @supportURL  <SupportUrl>
// @updateURL   <UpdateUrl>
// @run-at	document-start
// ==/UserScript==
(function() {var css = "";
if (false || (new RegExp("<Url_Regex>")).test(document.location.href))
    css += [
<Style>
    ].join("\n");
if (typeof GM_addStyle != "undefined") {
    GM_addStyle(css);
} else if (typeof PRO_addStyle != "undefined") {
    PRO_addStyle(css);
} else if (typeof addStyle != "undefined") {
    addStyle(css);
} else {
    var node = document.createElement("style");
    node.type = "text/css";
    node.appendChild(document.createTextNode(css));
    var heads = document.getElementsByTagName("head");
    if (heads.length > 0) {
        heads[0].appendChild(node);
    } else {
        // no head yet, stick it whereever
        document.documentElement.appendChild(node);
    }
}
})();"##
            .to_string();
    }
}
