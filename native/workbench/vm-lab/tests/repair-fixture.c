/* INTA offline UEFI regression fixture. Repairs only the intentionally corrupted BPB
   of the exact 1.44 MiB test volume with the matching label and marker. */
typedef unsigned long long U64;
typedef unsigned int U32;
typedef unsigned short U16;
typedef unsigned char U8;
typedef U64 S;
typedef void *H;
typedef struct { U32 a; U16 b, c; U8 d[8]; } GUID;
typedef struct { U8 header[24]; H method[40]; } SERVICES;
typedef struct {
    U32 id; U8 removable, present, logical, readonly, caching;
    U32 size, align; U64 last;
} MEDIA;
typedef struct BLOCK BLOCK;
typedef S (*IO)(BLOCK *, U32, U64, U64, void *);
struct BLOCK { U64 revision; MEDIA *media; H reset; IO read, write; S (*flush)(BLOCK *); };
typedef S (*LOCATE)(U32, GUID *, H, U64 *, H **);
typedef S (*GET)(H, GUID *, H *);
typedef void (*RESET)(U32, S, U64, H);
typedef struct { U8 before[88]; SERVICES *runtime, *boot; } SYSTEM;
static GUID block_id={0x964e5b21,0x6459,0x11d2,{0x8e,0x39,0,0xa0,0xc9,0x69,0x72,0x3b}};
static __declspec(align(4096)) U8 sector[512];
static const char *volatile label="INTALABCHK ";
static const char *volatile marker="INTA-OFFLINE-FIXTURE-V1";

static int matches(unsigned at,const char *text) {
    for(unsigned i=0;text[i];++i) if(sector[at+i]!=(U8)text[i]) return 0;
    return 1;
}
S efi_main(H image,SYSTEM *system) {
    (void)image;
    U64 n=0; H *handles=0;
    if(!((LOCATE)system->boot->method[36])(2,&block_id,0,&n,&handles)) {
        for(U64 i=0;i<n;i++){
            BLOCK *b=0;
            if(((GET)system->boot->method[16])(handles[i],&block_id,(H *)&b)||!b||!b->media) continue;
            MEDIA *m=b->media;
            if(!m->present||m->readonly||m->logical||m->size!=512||m->last!=2879)continue;
            if(b->read(b,m->id,0,512,sector)||!matches(43,label)||!matches(400,marker))continue;
            if(sector[11]||sector[12]||sector[510]!=0x55||sector[511]!=0xaa)continue;
            sector[12]=2;
            if(!b->write(b,m->id,0,512,sector)) b->flush(b);
            break;
        }
    }
    /* EfiResetShutdown. The host independently reads and validates the repaired overlay. */
    ((RESET)system->runtime->method[10])(2,0,0,0);
    return 0;
}
