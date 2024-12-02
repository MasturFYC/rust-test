<style lang="scss">
	div {
		margin-top: 12px;
		line-height: 1.5;
	}
	pre {
		padding: 12px;
		background-color: #dedede;
		line-height: 1.5;
		overflow-x: scroll;
	}
</style>

<script lang="ts">
	const title = 'Facebook Crawler';
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" content="Crawler this app" />
</svelte:head>

<h2>{title}</h2>

<div>
	Apa itu <strong>crawler</strong>, <code>crawler</code> adalah perayap yang digunakan
	untuk mengumpulkan, menyimpan cache, dan menampilkan informasi tentang aplikasi
	atau situs web seperti judul, deskripsi, dan gambar mini.
</div>
<div>
	Cara kerja Perayap ini merayapi sebuah halaman web. Kemudian menyimpan URL
	kanonis yang telah disimpan metadatanya dalam cache saat pertama kali
	seseorang membagikan atau menyukai suatu URL atau sebuah halaman web.
</div>
<div>
	Berikut ini adalah setingan yang digunakan pada server yang menggunakan
	<code>nginx</code> sebagai mesin web server:
</div>
<div>
	<code
		><pre>
# back-end service aktif lainnya
# upstream other-server &#123;
    # server YOUR_ADDRESS_01;
    # server YOUR_ADDRESS_02;
# &#125;

upstream shadow-servers &#123;
    # alamat server bayangan
    # jika dijalankan menggunakan docker-compose bersama nginx dan server
    # bayangan akan dibungkus dalam satu jaringan
    # bayangan adalah nama container, 7500 adalah port HTTP yang digunakan untuk
    # merespon request
    server bayangan:7500;
&#125;

server &#123;
    listen       80;
    server_name  localhost;

    root /var/www/fyc.com;

    location / &#123;
        try_files $uri @prerender;
    &#125;

    # untuk semua permintaan ke statis kita harus melakukan rendering server untuk perayap..
    location @prerender &#123;
    	set $server_side_render 0;
        
	# deteksi sebuah bot!
        # daftar bot web di bawah ini tidak lengkap tidak mencakup semua bot
	# hanya sebagian saja
        if ($http_user_agent ~* "discordbot|googlebot|twitterbot|facebookexternalhit|whatsapp|skypeuripreview") &#123;
            set $server_side_render 1;
        &#125;

        if ($args ~ "_escaped_fragment_") &#123;
            set $server_side_render 1;
        &#125;

        # abaikan jika ada permintaan pada file media tertentu
        if ($uri ~* "\.(js|css|xml|less|png|jpg|jpeg|gif|pdf|doc|txt|ico|rss|zip|mp3|rar|exe|wmv|doc|avi|ppt|mpg|mpeg|tif|wav|mov|psd|ai|xls|mp4|m4a|swf|dat|dmg|iso|flv|m4v|torrent|ttf|woff|svg|eot)") &#123;
            set $server_side_render 0;
        &#125;
        
	# selesaikan menggunakan server DNS Google untuk memaksakan resolusi DNS dan mencegah cache IP
        resolver 8.8.8.8;

        # jika url adalah web bot maka tulis ulang URL dan arahkan (redirect) ke server bayangan
        if ($server_side_render = 1) &#123;
            rewrite ^/(.*)$ /$1 break;
            proxy_pass http://shadow-servers;
        &#125;

        # try_files adalah instruksi untuk mengirim kembali halaman statis (SPA)
	# jika request datangnya bukan dari bot
        try_files $uri $uri/ /index.html;

    &#125;
&#125;
</pre>
	</code>
</div>
