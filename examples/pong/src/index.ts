import * as laser from 'laserlight/src_ts/index';

(async () => {
    laser.fancyUpDocument();
    await laser.init('pong');
})();
