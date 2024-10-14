# create-svelte

1) Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/master/packages/create-svelte).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```sh
# create a new project in the current directory
npm create svelte@latest

# create a new project in my-app
npm create svelte@latest my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.
> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

> To run :

```
npm run prepare
npm run dev
```

references

```
https://hartenfeller.dev/blog/sveltekit-with-sqlite
```

### ssh-copy-id
<ol>
<li> generate 4k key

```
ssh-keygen -t rsa -b 4096 -f ~/.ssh/id_rsa4k
```
<li> create ssh-agent-pid

```
ssh-agent -s > ~/mysshagent
source ~/mysshagent
rm ~/mysshagent
```
<li> loading key

```
ssh-add -l
ssh-add -L
```
<li> try login to host

```
ssh <user>@<host>
```
<li> copy ssh key

```
ssh-copy-id <user>@<host>
```
<li> try login again (success if no prompt for password)

```
ssh <user>@<host>
```

facebook problem with shared link
https://stackoverflow.com/questions/70030269/reactjs-image-preview-problem-of-the-shared-post-on-facebook
