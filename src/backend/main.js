const express = require('express');
const app = express();
const fs = require('fs');
require('dotenv').config();
const path = require('path');
const pinataSDK = require('@pinata/sdk');
const pinata = new pinataSDK(process.env.API_KEY, process.env.API_SECRET_KEY);
const { nearConnection, getAccount, create_work } = require('./near-middlewares');

app.set("views", path.join(__dirname, "..", "..", "src", "frontend", "views"));
app.set('view engine', 'ejs');

const multer = require('multer');

const storage = multer.diskStorage({
    destination: async (req, file, cb) => {
        let fileName = path.join(__dirname, '..', '..', 'resources', 'uploads');

        if (!fs.existsSync(fileName)) {
            await fs.promises.mkdir(fileName);
        }

        cb(null, fileName);
    },
    filename: (req, file, cb) => {
        cb(null, file.originalname);
    }
});

const supportedFile = ['docx', 'txt', 'png', 'jpg', 'jpeg', 'mp3', 'wav', 'pdf', 'mp4'];
const fileFilter = (req, file, cb) => {
    let extension = file.originalname.split('.')[1];

    if (!supportedFile.includes(extension)) {
        cb(null, false);
    } else {
        cb(null, true);
    }
}

const uploadConfig = multer({ storage, fileFilter });
const uploader = uploadConfig.single('fileInput');

app.use(express.json());

app.post('/upload', uploader, async (req, res) => {
    let file = req.file;
    let name = req.body.name;
    let accountId = req.body.accountId;
    let fee = req.body.fee;
    

    if (!name || !accountId) {
        return res.status(400).json({ code: 0, msg: 'Invalid information' });
    }

    if (!file) {
        return res.status(400).json({ code: 0, msg: 'No file found' });
    }
    
    try {
        nearConnection();
        let account = await getAccount(accountId);
        
        let readableStreamForFile = fs.createReadStream(file.path);

        let options = {
            pinataMetadata: {
                name: file.originalname,
            },
            pinataOptions: {
                cidVersion: 0
            }
        };

        let data = await pinata.pinFileToIPFS(readableStreamForFile, options);
        data = { link: `https://gateway.pinata.cloud/ipfs/${data.IpfsHash}`, ...data };
        
        await create_work(account, name, data.link, Number(fee));

        return res.redirect(data.link);
    } catch (error) {
        console.log("Error Message: ", error);
        return res.status(500).json({ code: 0, msg: "There is something wrong" });
    }
});

app.delete('/delete', (req, res) => {
    let cid = req.body.cid;
    pinata.unpin(cid)
        .then(result => res.status(200).json({ code: 1, msg: 'Success' }))
        .catch(error => {
            console.log(error);
            return res.status(500).json({ code: 0, msg: `Error: ${error}` });
        })
});

app.get('/', (req, res) => res.render('index'));

let PORT = process.env.PORT || 8080;
app.listen(PORT, () => console.log(`http://localhost:${PORT}`));