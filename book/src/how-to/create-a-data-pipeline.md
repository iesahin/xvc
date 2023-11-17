# How to create a data pipeline with Xvc

A data pipeline starts from data and ends with models. 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```


```console
$ tree data
data
├── contracts
│   ├── A Consulting Agreement- Consumer Recreations Services V10.DOC
│   ├── AGREEMENT TO SETTLE (BCHRT).docx
│   ├── House-Rental-Contract (HLoom).docx
│   ├── Investment-Contract (HLoom).docx
│   ├── Limited Warranty (Pro remodeler).docx
│   ├── Mutual Confidentiality Agreement Blue sun & Stay Puft V8docx.docx
│   ├── Non-Compete (Signaturely).docx
│   ├── Project-Manager-Contract (Hloom).docx
│   ├── Roofing Contract (Signaturely).docx
│   ├── Services Contract -Cyberdyne Systems V12.docx
│   ├── Website Work-for_hire (Signaturely).docx
│   └── XYZ Corp Employment Agreement.docx
└── non-contracts
    ├── 10 steps for marketing your law firm.docx
    ├── 20+ Future Business in India for 2025 _ Future Business Ideas for 2030 and beyond.docx
    ├── Determining Culture Fit.docx
    ├── How Does Working In-House Differ from Private Practice_.docx
    ├── Invoice (HLoom).docx
    ├── Is Remote Work Working.docx
    ├── Women who broke barriers in the music industry.docx
    └── invoice-spiceimporter.docx

3 directories, 20 files

```



```console
$ xvc file track data

$ xvc pipeline step new -s convert-docx-to-txt --command "./convert-docx-to-txt.zsh" 
```


