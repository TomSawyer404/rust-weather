# rust-weather

when you type `weather <city>`, it will show you the weather of the city you input.

Useful API comes from [openweathermap.org](https://openweathermap.org/api). You can register
an account and get API KEY. `4845f22236e074cdac59ae174aa580a3`is the KEY I will use.
After you get the KEY, you can type:

```url
http://api.openweathermap.org/data/2.5/weather?q=Beijing&appid=4845f22236e074cdac59ae174aa580a3
```

in you brower url. and then the server will return you a json data.

The project is inspired by this [video](https://www.bilibili.com/video/BV1eL411b7EL?from=search&seid=15342924168800539932&spm_id_from=333.337.0.0).

