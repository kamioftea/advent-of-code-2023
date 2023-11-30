const highlightJs = require('highlight.js')
const  md = require('markdown-it')()

highlightJs.configure({})

module.exports = function (string, language) {
    if (language && highlightJs.getLanguage(language)) {
        try {
            return '<pre class="hljs"><code class="code-block">' +
                highlightJs.highlight(string, { language, ignoreIllegals: true }).value +
                '</code></pre>';
        } catch (__) {}
    }

    return '<pre class="hljs"><code class="code-block">' + md.utils.escapeHtml(string) + '</code></pre>';
}
