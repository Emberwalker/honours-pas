<template>
<div class="project-editor">
  <div class="row">
    <div class="col">
      <h3>Metadata</h3>
      <div class="form-group row">
        <label class="col-sm-2 col-form-label" for="metadata-name">Project name</label>
        <div class="col-sm-10">
          <input v-model="project.name" type="text" class="form-control" id="metadata-name"/>
        </div>
      </div>
      <div class="form-group row">
        <label class="col-sm-2 col-form-label" for="metadata-supervisor-name">Project supervisor</label>
        <div class="input-group col-sm-10">
          <div v-if="!allowAuthorChanges" class="input-group-addon">(Read-only)</div>
          <input v-model="project.supervisor_name" type="text" class="form-control" id="metadata-supervisor-name"
                 :readonly="!allowAuthorChanges"/>
        </div>
      </div>
      <div class="form-group row">
        <label class="col-sm-2 col-form-label" for="metadata-supervisor-email">Supervisor email</label>
        <div class="input-group col-sm-10">
          <div v-if="!allowAuthorChanges" class="input-group-addon">(Read-only)</div>
          <input v-model="project.supervisor_email" type="text" class="form-control" id="metadata-supervisor-email"
                 :readonly="!allowAuthorChanges"/>
        </div>
      </div>
      <div class="form-group row">
        <label class="col-sm-2 col-form-label">Additional staff</label>
        <div class="input-group col-sm-4">
          <div class="input-group-addon" id="metadata-additional-name-label">Name</div>
          <input v-model="additional_name" type="text" class="form-control" aria-labelledby="metadata-additional-name-label"/>
        </div>
        <div class="input-group col-sm-4">
          <div class="input-group-addon" id="metadata-additional-email-label">Email</div>
          <input v-model="additional_email" type="email" class="form-control" aria-labelledby="metadata-additional-email-label"/>
        </div>
        <div class="col-sm-2">
          <button @click="addStaffer" class="add-staff-btn btn btn-success btn-sm">Add</button>
        </div>
      </div>
      <div class="row">
        <p class="text-muted form-hint">For additional staff, click 'Add' to commit a new entry to the list.</p>
      </div>
      <div v-if="project.additional_staff.length > 0" class="form-group row">
        <div class="col-sm-2"></div>
        <div class="col-sm-10">
          <ul class="list-group">
            <li v-for="staffer in project.additional_staff" class="list-group-item">
              <span>{{ staffer }}</span>
              <button @click="removeStaffer(staffer)" class="btn btn-danger btn-sm btn-right">Delete</button>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
  <!-- Description -->
  <div class="row">
    <div class="col-lg">
      <h3 id="description_label">Description</h3>
      <textarea title="project description" aria-labelledby="description_label" @input="onUpdate" class="form-control" rows="26">{{ project.description_md }}</textarea>
    </div>
    <div class="col-lg-4 help-col">
      <div class="card">
        <h3 class="card-header bg-info text-white"><feather icon="edit-2"/>Formatting</h3>
        <div class="card-body">
          <h5 class="text-muted">A Markdown Primer</h5>
          <p>Markdown is a plain text-based formatting system commonly used by developers on platforms such as GitHub.</p>
          <p>
            Below is a list of common operations. For a more complete reference, see the
            <a href="https://guides.github.com/features/mastering-markdown/" target="_blank">GitHub Guide 'Mastering Markdown'</a>.
          </p>
          <ul class="markdown-list">
            <li><code># Text</code>: Header</li>
            <li><code>*text*</code> or <code>_text_</code>: Italics</li>
            <li><code>**text**</code>: Bold</li>
            <li><code>[Link Text](http://example.com)</code>: Hyperlinks</li>
            <li><code>`text`</code>: Inline code block</li>
            <li><code>```java</code>: Start Java syntax code block</li>
            <li><code>```</code>: End code block</li>
          </ul>
          <p class="markdown-footnote">
            Note that not all features of GitHub Flavoured Markdown are supported - GitHub-specific features such as
            SHA-1 linking are not provided. Tables, syntax highlighting and automatic linking are supported. Raw HTML
            is escaped.
          </p>
        </div>
      </div>
    </div>
  </div>
  <!-- Description END -->
  <!-- Preview -->
  <div class="row">
    <div class="col">
      <h3>Preview</h3>
      <p class="text-muted">Note that the preview may take a second to update.</p>
      <project-card :project="project"></project-card>
    </div>
  </div>
  <!-- Preview END -->
  <div class="row">
    <div class="col">
      <button v-on:click="onDone" class="btn btn-lg btn-primary">Save</button>
    </div>
  </div>
</div>
</template>

<script lang="ts">
import $ from "jquery";
import _ from "lodash";
import Vue from "vue";
import { parseMarkdown } from "../lib/Util";
import ProjectCard from "./ProjectCard.vue";

export default Vue.extend({
  components: {
    "project-card": ProjectCard,
  },
  computed: {},
  data() {
    // Create a *copy* of the project so we don't accidentally overwrite state.
    const project = $.extend({}, this.$props.initialProject);
    return {
      additional_email: "",
      additional_name: "",
      project,
    };
  },
  methods: {
    onDone() {
      this.$emit("edit-complete", this.project);
    },
    onUpdate: _.debounce(function(this: any, e: any) {
      this.project.description_md = e.target.value;
    }, 250),
    addStaffer() {
      if (this.additional_name.length > 0 && this.additional_email.length > 0) {
        this.project.additional_staff = _.union(this.project.additional_staff,
                                                [this.additional_name + " <" + this.additional_email + ">"]);
        this.additional_name = "";
        this.additional_email = "";
      }
    },
    removeStaffer(staffer: string) {
      this.project.additional_staff = _.reject(this.project.additional_staff, (ent) => ent === staffer);
    },
  },
  name: "ProjectEditor",
  props: {
    allowAuthorChanges: {
      default: false,
      required: false,
      type: Boolean,
    },
    initialProject: {
      required: true,
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
  .row {
    margin-bottom: 1rem;
  }

  .btn-right {
    float: right;
  }

  .add-staff-btn {
    // Fix button alignment
    top: 25%;
  }

  .form-hint {
    margin-left: 0.9rem;
  }

  .markdown-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .markdown-footnote {
    margin-top: 1rem;
    margin-bottom: 0;
  }
</style>
