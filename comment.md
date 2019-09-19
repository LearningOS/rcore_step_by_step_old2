<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/gitalk@1/dist/gitalk.css"
/>
<script src="https://cdn.jsdelivr.net/npm/gitalk@1/dist/gitalk.min.js"></script>
<div id="gitalk-container"></div>
<script>
  const gitalk = new Gitalk({
    clientID: '6fb135ae71db34448a92',
    clientSecret: '60f7c70e9473e5a158e84d91865ef76212938468',
    repo: 'rcore_step_by_step',
    owner: 'xy-plus',
    admin: ['xy-plus']
  })

gitalk.render('gitalk-container')
</script>
