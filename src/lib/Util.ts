import hljs from "highlight.js";
import * as marked from "marked";

export function highlightingInit() {
  marked.setOptions({
    highlight(code, lang) {
      if (lang) {
        return hljs.highlight(lang, code, true).value;
      }
      return hljs.highlightAuto(code).value;
    },
    sanitize: true,
  });
}

export function parseMarkdown(md: string): string {
  const out = marked(md);
  return out;
}

export function renderCodeBlock(el: Node) {
  hljs.highlightBlock(el);
}
