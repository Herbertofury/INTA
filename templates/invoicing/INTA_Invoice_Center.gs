/**
 * INTA Invoice Center — bound Google Apps Script, v2.0.0.
 * Install in either INTA Quick Invoice or Invoice Studio PRO.
 * No automatic sending, triggers, public sharing or client data at installation.
 */
const INTA_VERSION = '2.1.0';
const INTA_RANGES = {
  quick: ['B6','B7','B8','B9','B10','B11','B12','B13','G6','G7','G10','G11','G12','B14','G14','B27','G27','A29','A33','G73'],
  studio: ['A6','C6','E6','G6','I6','K6','M6','B7','F7','B10','D10','F10','H10','J10','L10','A25','C25','A28','H28']
};
function onOpen() {
  const ui=SpreadsheetApp.getUi(), m=ui.createMenu('INTA Invoice');
  m.addItem('New invoice','intaNewInvoice').addItem('Generate invoice number','intaNumber').addItem('Duplicate archived invoice','intaDuplicate')
   .addSeparator().addSubMenu(ui.createMenu('Recurring billing').addItem('Add monthly preset (Quick)','intaMonthlyPreset').addItem('Load monthly contract (Studio)','intaLoadContract'))
   .addSubMenu(ui.createMenu('Documents').addItem('Open invoice','intaOpenInvoice').addItem('Convert estimate to invoice','intaConvertEstimate').addItem('Prepare receipt','intaPrepareReceipt').addItem('Open document editor','intaOpenDocuments').addItem('Open account statement','intaOpenStatement'))
   .addSeparator().addItem('Create client-only copy / share link','intaClientCopy').addItem('Export PDF','intaExportPdf').addItem('Send approved document email','intaSendEmail').addItem('Archive invoice + PDF','intaArchive')
   .addSeparator().addItem('Record payment or refund','intaPayment').addItem('Draft overdue reminder (Studio)','intaReminder').addItem('Reset editor','intaReset').addToUi();
}
function intaBook(){return SpreadsheetApp.getActiveSpreadsheet();}
function intaUi(){return SpreadsheetApp.getUi();}
function intaCfg(){
  const book=intaBook(), quick=book.getSheetByName('Quick Edit');
  if(quick && book.getSheetByName('Invoice'))return {book:book,kind:'quick',editor:quick,invoice:book.getSheetByName('Invoice'),id:'B12',date:'B13'};
  const b=book.getSheetByName('Builder');
  if(b&&book.getSheetByName('Register')&&book.getSheetByName('Invoice'))return {book:book,kind:'studio',editor:b,invoice:book.getSheetByName('Invoice'),id:'C6',date:'E6'};
  throw new Error('Open the private INTA Quick Invoice or Studio PRO workbook, not a client copy.');
}
function intaRun(fn){try{return fn();}catch(e){intaUi().alert('INTA Invoice',String(e.message||e),intaUi().ButtonSet.OK);}}
function intaAsk(title,msg){const ui=intaUi(),r=ui.prompt(title,msg,ui.ButtonSet.OK_CANCEL);return r.getSelectedButton()===ui.Button.OK?r.getResponseText().trim():null;}
function intaYes(title,msg){const u=intaUi();return u.alert(title,msg,u.ButtonSet.YES_NO)===u.Button.YES;}
function intaGet(s,a){return s.getRange(a).getValue();}
function intaText(s,a){return String(s.getRange(a).getDisplayValue()).trim();}
function intaSet(s,a,v){const r=s.getRange(a);if(v===''||v===null||v===undefined){const merged=r.getMergedRanges();(merged.length?merged[0]:r).clearContent();}else{r.setValue(typeof v==='string'&&v.charAt(0)==='='?"'"+v:v);}}
function intaCents(n){return Math.round(Number(n)*100);}
function intaMoney(n){return '$'+(Math.round(Number(n)*100)/100).toFixed(2);}
function intaPositive(raw){if(raw===null||raw===''||!Number.isFinite(Number(raw))||Number(raw)<=0)throw new Error('Enter a positive amount.');return intaCents(raw)/100;}
function intaDate(raw){const m=/^(\d{4})-(\d{2})-(\d{2})$/.exec(String(raw));if(!m)throw new Error('Use YYYY-MM-DD.');const d=new Date(+m[1],+m[2]-1,+m[3],12);if(d.getFullYear()!==+m[1]||d.getMonth()!==+m[2]-1||d.getDate()!==+m[3])throw new Error('Invalid date.');return d;}
function intaEmail(raw){return typeof raw==='string'&&/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(raw);}
function intaUrl(raw){return typeof raw==='string'&&/^https:\/\/[^\s]+$/i.test(raw);}
function intaHistory(c){
  let h=c.book.getSheetByName('_Invoice History');
  if(!h){h=c.book.insertSheet('_Invoice History');h.getRange(1,1,1,8).setValues([['Invoice ID','Issued','Client','Draft snapshot JSON','Client sheet','PDF','Total','Savings']]);h.setFrozenRows(1);h.hideSheet();}
  return h;
}
function intaFindKey(sh,id,start){
  if(!sh||sh.getMaxRows()<start)return 0;
  const a=sh.getRange(start,1,sh.getMaxRows()-start+1,1).getDisplayValues();
  const i=a.findIndex(x=>String(x[0]).trim()===String(id).trim());return i<0?0:i+start;
}
function intaNewId(c){
  const props=PropertiesService.getDocumentProperties(),tz=c.book.getSpreadsheetTimeZone(),year=Utilities.formatDate(new Date(),tz,'yyyy');
  const pre=c.kind==='studio'?intaText(c.book.getSheetByName('Settings'),'B12')||'INTA':'INTA';
  const prefix=pre.replace(/[^A-Za-z0-9_-]/g,'').slice(0,24)||'INTA';
  let n=Number(props.getProperty('INTA_SEQ_'+year)||0),id;
  const hist=c.book.getSheetByName('_Invoice History'),reg=c.book.getSheetByName('Register');
  do{id=prefix+'-'+year+'-'+String(++n).padStart(4,'0');}while(intaFindKey(hist,id,2)||intaFindKey(reg,id,5));
  props.setProperty('INTA_SEQ_'+year,String(n));return id;
}
function intaLock(fn){const l=LockService.getDocumentLock()||LockService.getScriptLock();l.waitLock(30000);try{return fn();}finally{l.releaseLock();}}
function intaNumber(){return intaRun(()=>{const c=intaCfg();if(intaText(c.editor,c.id)&&!intaYes('Replace invoice number?','Generate a new unique number for the current draft?'))return;intaLock(()=>intaSet(c.editor,c.id,intaNewId(c)));if(!intaGet(c.editor,c.date))intaSet(c.editor,c.date,new Date());});}
function intaHasDraft(c){const sh=c.editor,a=c.kind==='quick'?sh.getRange('A17:A26').getDisplayValues():sh.getRange('A13:B22').getDisplayValues();return Boolean(intaText(sh,c.id)||a.some(r=>r.some(Boolean)));}
function intaClear(c){
  const s=c.editor, comparison=c.book.getSheetByName('Comparable Rates');
  if(comparison)comparison.getRange('E5:H14').setValues(Array.from({length:10},()=>['','Auto','','']));
  if(c.kind==='quick'){
    ['B6','B7','B8','B9','B10','B11','B12','B13','A29','G73'].forEach(a=>intaSet(s,a,''));
    s.getRange('A17:F26').setValues(Array.from({length:10},()=>['','','Auto','Auto',0,'Auto']));
    s.getRange('G53:G62').clearContent();s.getRange('H53:H62').setValues(Array.from({length:10},()=>[.2]));
    intaSet(s,'G6','Residential');intaSet(s,'G7','Auto');intaSet(s,'G11',0);intaSet(s,'G13',0);intaSet(s,'B27','Standard');intaSet(s,'G27','None');
  }else{
    ['A6','C6','E6','G6','I6','K6','M6','B7','F7','A28','C25'].forEach(a=>intaSet(s,a,''));
    s.getRange('A13:G22').clearContent();s.getRange('B40:B49').clearContent();s.getRange('C40:C49').setFormula('=Settings!B27');
    intaSet(s,'B10','Auto');intaSet(s,'D10','Catalog');intaSet(s,'J10','Standard');intaSet(s,'L10','None');intaSet(s,'A25',0);s.getRange('H28').setFormula('=Settings!B8');
  }
}
function intaReset(){return intaRun(()=>{const c=intaCfg();if(intaYes('Reset editor','Clear current draft inputs? Saved invoice files and history will remain unchanged.'))intaClear(c);});}
function intaNewInvoice(){return intaRun(()=>{const c=intaCfg();if(intaHasDraft(c)&&!intaYes('New invoice','Replace the current unarchived draft? Archive it first if needed.'))return;intaLock(()=>{intaClear(c);intaSet(c.editor,c.id,intaNewId(c));intaSet(c.editor,c.date,new Date());});c.editor.activate();});}
function intaEncode(v){return v instanceof Date?{date:v.toISOString()}:v;}
function intaDecode(v){return v&&typeof v==='object'&&v.date?new Date(v.date):v;}
function intaCapture(c){
  const s=c.editor, cells={};
  INTA_RANGES[c.kind].forEach(a=>cells[a]=intaEncode(intaGet(s,a)));
  const rows=c.kind==='quick'?'A17:F26':'A13:G22',cost=c.kind==='quick'?'G53:H62':'B40:C49';
  const comp=c.book.getSheetByName('Comparable Rates');
  return {version:INTA_VERSION,kind:c.kind,cells:cells,rows:s.getRange(rows).getValues().map(r=>r.map(intaEncode)),cost:s.getRange(cost).getValues().map(r=>r.map(intaEncode)),comparison:comp?{lines:comp.getRange('A5:K14').getValues().map(r=>r.map(intaEncode)),matchedNet:intaGet(comp,'A17'),estimate:intaGet(comp,'D17'),savings:intaGet(comp,'G17'),source:intaText(comp,'A19'),sourcePages:intaText(comp,'A21')}:null,payee:intaText(c.invoice,'B33')};
}
function intaDuplicate(){return intaRun(()=>{
  const c=intaCfg(),h=c.book.getSheetByName('_Invoice History');if(!h)throw new Error('Archive an invoice first.');
  const id=intaAsk('Duplicate invoice','Enter an archived invoice number.');if(id===null)return;
  const r=intaFindKey(h,id,2);if(!r)throw new Error('Invoice is not in this workbook history.');
  const d=JSON.parse(h.getRange(r,4).getValue());if(d.kind!==c.kind)throw new Error('Archive belongs to a different workbook type.');
  if(intaHasDraft(c)&&!intaYes('Duplicate invoice','Replace the current draft with a new copy of '+id+'? Comparable-rate selections reset; reselect only estimates that still match the work.'))return;
  intaLock(()=>{intaClear(c);Object.keys(d.cells).forEach(a=>intaSet(c.editor,a,intaDecode(d.cells[a])));
    c.editor.getRange(c.kind==='quick'?'A17:F26':'A13:G22').setValues(d.rows.map(r=>r.map(intaDecode)));
    c.editor.getRange(c.kind==='quick'?'G53:H62':'B40:C49').setValues(d.cost.map(r=>r.map(intaDecode)));
    intaSet(c.editor,c.id,intaNewId(c));intaSet(c.editor,c.date,new Date());if(c.kind==='quick')intaSet(c.editor,'G13',0);});
  c.editor.activate();
});}
function intaMonthlyPreset(){return intaRun(()=>{
  const c=intaCfg();if(c.kind!=='quick')throw new Error('Use Load monthly contract or select a monthly service in Studio Builder.');
  const opt=intaAsk('Monthly preset','Enter 500 (on-call), 900 (managed IT), or CUSTOM (agreed monthly amount).');if(opt===null)return;
  const map={'500':'Monthly On-call Retainer ($500)','900':'Managed IT Support ($900/month)','CUSTOM':'Monthly IT Support — Custom'},key=opt.toUpperCase();
  if(!map[key])throw new Error('Choose 500, 900 or CUSTOM.');
  let rate=null;if(key==='CUSTOM'){const x=intaAsk('Monthly amount','Enter the agreed monthly fee.');if(x===null)return;rate=intaPositive(x);}
  const sh=c.editor,rows=sh.getRange('A17:F26').getDisplayValues(),i=rows.findIndex(r=>!r[0]&&!r[1]&&r[2]==='Auto'&&r[3]==='Auto'&&Number(r[4]||0)===0&&r[5]==='Auto');
  if(i<0)throw new Error('No unused invoice line is available.');const r=i+17;
  sh.getRange(r,1,1,6).setValues([[map[key],1,'Auto',rate===null?'Auto':rate,0,'Auto']]);sh.activate();
});}
function intaLoadContract(){return intaRun(()=>{
  const c=intaCfg();if(c.kind!=='studio')throw new Error('Use Monthly preset in Quick Invoice.');
  const contracts=c.book.getSheetByName('Contracts'),reg=c.book.getSheetByName('Register');
  let id=intaText(c.editor,'B7');if(!id){id=intaAsk('Monthly contract','Enter an existing active contract ID from Contracts.');if(id===null)return;}
  const r=intaFindKey(contracts,id,5);if(!r)throw new Error('Unknown contract ID.');
  const d=contracts.getRange(r,1,1,17).getValues()[0];if(d[8]!=='Active'||!d[1]||d[3]===''||!Number.isFinite(Number(d[3])))throw new Error('Select an active contract with a client and agreed monthly fee.');
  if(!(d[6] instanceof Date))throw new Error('Contract requires a period start date.');
  const tz=c.book.getSpreadsheetTimeZone(),defaultPeriod=Utilities.formatDate(d[6],tz,'yyyy-MM-dd');
  const raw=intaAsk('Billing period','Enter period start YYYY-MM-DD, or leave blank for '+defaultPeriod+'.');if(raw===null)return;const period=raw?intaDate(raw):d[6];
  if(intaContractAlreadyBilled(reg,id,period,tz))throw new Error('This contract and billing period already exist in Register.');
  if(intaHasDraft(c)&&!intaYes('Load contract','Replace the current draft with this contract billing period?'))return;
  intaLock(()=>{intaClear(c);intaSet(c.editor,'A6',d[1]);intaSet(c.editor,'B7',id);intaSet(c.editor,'F7',period);intaSet(c.editor,c.id,intaNewId(c));intaSet(c.editor,c.date,new Date());intaSet(c.editor,'A13','Contract monthly fee');SpreadsheetApp.flush();if(Number(intaGet(c.editor,'L7'))>0)intaSet(c.editor,'A14','Contract overage');});
  c.editor.activate();
});}
function intaContractAlreadyBilled(reg,id,period,tz){
  const data=reg.getRange(5,13,reg.getMaxRows()-4,2).getValues(),key=Utilities.formatDate(period,tz,'yyyy-MM-dd');
  return data.some(r=>r[0]===id&&r[1] instanceof Date&&Utilities.formatDate(r[1],tz,'yyyy-MM-dd')===key);
}
function intaOpenInvoice(){return intaRun(()=>intaCfg().invoice.activate());}
function intaOpenDocuments(){return intaRun(()=>{const c=intaCfg();if(c.kind!=='studio')throw new Error('Document Center is in Studio PRO.');c.book.getSheetByName('Documents').activate();});}
function intaOpenStatement(){return intaRun(()=>{const c=intaCfg();if(c.kind!=='studio')throw new Error('Statements are in Studio PRO.');c.book.getSheetByName('Statement').activate();});}
function intaConvertEstimate(){return intaRun(()=>{const c=intaCfg();if(c.kind!=='studio'||intaText(c.book.getSheetByName('Documents'),'C5')!=='Estimate')throw new Error('Select Estimate in Documents first.');if(!intaYes('Convert estimate','Use the same client and priced lines to create a new invoice number?'))return;intaLock(()=>{intaSet(c.editor,c.id,intaNewId(c));intaSet(c.editor,c.date,new Date());});c.invoice.activate();});}
function intaPrepareReceipt(){return intaRun(()=>{
  const c=intaCfg();if(c.kind!=='studio')throw new Error('Receipts are in Studio PRO.');
  const id=intaText(c.editor,c.id),paid=Number(intaGet(c.invoice,'H33'));if(!id||paid<=0)throw new Error('Select an invoice with a confirmed recorded payment.');
  const raw=intaAsk('Receipt amount','Enter the confirmed payment amount (net recorded payments '+intaMoney(paid)+').');if(raw===null)return;const amount=intaPositive(raw);if(amount>paid)throw new Error('Receipt exceeds recorded payments.');
  const method=intaAsk('Payment method','Enter the confirmed payment method or reference.');if(method===null)return;
  const d=c.book.getSheetByName('Documents');intaSet(d,'C5','Receipt');intaSet(d,'C6','RCPT-'+id+'-'+Utilities.formatDate(new Date(),c.book.getSpreadsheetTimeZone(),'yyyyMMdd-HHmmss'));intaSet(d,'C7',id);intaSet(d,'C8',amount);intaSet(d,'C9',method);intaSet(d,'I6',new Date());d.activate();
});}
function intaPrintable(c){
  const a=c.book.getActiveSheet().getName();
  if(c.kind==='studio'&&(a==='Document'||a==='Documents'))return c.book.getSheetByName('Document');
  if(c.kind==='studio'&&a==='Statement')return c.book.getSheetByName('Statement');
  return c.invoice;
}
function intaReady(c,sh){
  SpreadsheetApp.flush();
  const name=sh.getName();
  if(name==='Statement'){if(!intaText(sh,'C4')||!intaText(sh,'E4'))throw new Error('Select a valid client ID on Statement.');return;}
  if(name==='Document'){
    const p=intaText(c.book.getSheetByName('Documents'),'A32');
    if(!/^REVIEW DOCUMENT BEFORE SHARING|^EXTERNAL INVOICE/.test(p))throw new Error(p||'Complete document fields.');
    if(/^EXTERNAL INVOICE/.test(p)&&!intaYes('External invoice','Confirm the client and amount are correct for this external invoice.'))throw new Error('Document not approved.');
    return;
  }
  const s=c.editor;
  if(!intaText(sh,'G2')||!intaText(sh,'F8')||!(intaGet(sh,'C14') instanceof Date))throw new Error('Add invoice ID, client and issue date.');
  const input=c.kind==='quick'?s.getRange('A17:A26').getDisplayValues():s.getRange('A13:B22').getDisplayValues();
  const out=sh.getRange('H17:H26').getValues();let count=0;
  input.forEach((r,i)=>{if(r.some(x=>x!=='')){if(typeof out[i][0]!=='number'||!Number.isFinite(out[i][0]))throw new Error('Complete service line '+(i+1)+', including quantity and rate.');count++;}});
  if(!count)throw new Error('Add at least one service line.');
  if(!intaText(sh,'B33'))throw new Error('Enter the payee and payment instructions.');
  const comp=c.book.getSheetByName('Comparable Rates');
  if(comp){const inputs=comp.getRange('E5:I14').getValues();inputs.forEach((r,i)=>{if((r[0]!==''||r[2]!==''||r[3]!=='')&&(typeof r[4]!=='number'||!Number.isFinite(r[4])))throw new Error('Complete or clear comparable estimate on line '+(i+1)+'.');});}
  if(c.kind==='studio'&&intaText(s,'B7')&&intaText(s,'N7')!=='Active')throw new Error('Contract status or client does not match.');
  if(c.kind==='studio'){
    const reg=c.book.getSheetByName('Register'),row=intaFindKey(reg,intaText(sh,'G2'),5);
    if(row){const a=reg.getRange(row,1,1,21).getValues()[0];
      if(a[20]!==intaText(s,'A6')||intaCents(a[5])!==intaCents(intaGet(sh,'H32')))throw new Error('Archived invoice differs from this draft. Use a new invoice number or the archived client copy.');
    }
  }
}
function intaPaymentOptions(c){if(c.kind==='quick')return {url:intaText(c.editor,'G68'),qr:intaText(c.editor,'G71')==='Yes'};const s=c.book.getSheetByName('Settings');return {url:intaText(s,'B24'),qr:intaText(s,'B25')==='Yes'};}
function intaTitle(c,sh){if(sh.getName()==='Statement')return 'INTA Account Statement - '+intaText(sh,'E4');const id=intaText(sh,'G2'),client=intaText(sh,'F8'),kind=sh.getName()==='Invoice'?'Invoice':intaText(c.book.getSheetByName('Documents'),'C5');return 'INTA '+kind+' '+id+' - '+client;}
function intaSetLink(sh,a,url){if(!intaUrl(url))return;const r=sh.getRange(a),text=r.getDisplayValue();if(text)r.setRichTextValue(SpreadsheetApp.newRichTextValue().setText(text).setLinkUrl(url).build());}
function intaSnapshot(c,sh){
  intaReady(c,sh);const statement=sh.getName()==='Statement',rows=statement?Math.max(11,sh.getLastRow()):40,cols=statement?8:10,area=sh.getRange(1,1,rows,cols),values=area.getValues(),display=area.getDisplayValues();
  const bad=/^#(REF!|ERROR!|VALUE!|N\/A|DIV\/0!|NUM!|NAME\?|NULL!)$/;
  if(display.some(r=>r.some(x=>bad.test(x))))throw new Error('Fix formula errors before creating a client copy.');
  const merges=area.getMergedRanges().map(r=>[r.getRow(),r.getColumn(),r.getNumRows(),r.getNumColumns()]);
  const title=intaTitle(c,sh).replace(/[\r\n]/g,' ').slice(0,180),out=SpreadsheetApp.create(title),dst=sh.copyTo(out);
  dst.setName(sh.getName());out.getSheets().filter(x=>x.getSheetId()!==dst.getSheetId()).forEach(x=>out.deleteSheet(x));
  [SpreadsheetApp.ProtectionType.SHEET,SpreadsheetApp.ProtectionType.RANGE].forEach(t=>dst.getProtections(t).forEach(p=>{if(p.canEdit())p.remove();}));
  const all=dst.getRange(1,1,rows,cols);all.breakApart();all.clearDataValidations();all.clearNote();
  all.setValues(values.map(r=>r.map(x=>typeof x==='string'&&x.charAt(0)==='='?"'"+x:x)));
  merges.forEach(m=>dst.getRange(m[0],m[1],m[2],m[3]).merge());
  if(dst.getMaxRows()>rows)dst.deleteRows(rows+1,dst.getMaxRows()-rows);
  if(dst.getMaxColumns()>cols)dst.deleteColumns(cols+1,dst.getMaxColumns()-cols);
  dst.setConditionalFormatRules([]);dst.setHiddenGridlines(true);dst.setFrozenRows(0);dst.setFrozenColumns(0);
  if(!statement){const opt=intaPaymentOptions(c);intaSetLink(dst,sh.getName()==='Invoice'?'B12':'B11','https://intacomputerservices.com');
    if(sh.getName()==='Invoice'&&intaUrl(opt.url)){intaSetLink(dst,'B39',opt.url);
      if(opt.qr){
        // Explicitly opted in; the payment URL is transmitted to the QR image provider.
        const res=UrlFetchApp.fetch('https://quickchart.io/qr?size=180&margin=2&text='+encodeURIComponent(opt.url),{muteHttpExceptions:true});
        if(res.getResponseCode()!==200)throw new Error('QR provider failed; the new copy remains private.');
        const note=dst.getRange('B33').getValue();dst.getRange('B33:D35').breakApart();dst.getRange('B33:C35').merge().setValue(note).setWrap(true);
        const img=dst.insertImage(res.getBlob(),4,33);img.setWidth(65).setHeight(65).setAnchorCellXOffset(3).setAnchorCellYOffset(2);
      }
    }
  }
  SpreadsheetApp.flush();
  if(dst.getDataRange().getFormulas().some(r=>r.some(Boolean)))throw new Error('Snapshot contains an unexpected formula; copy stays private.');
  return {book:out,sheet:dst,title:title,url:out.getUrl(),statement:statement};
}
function intaShareCopy(snap,access){
  if(!access)return;
  const f=DriveApp.getFileById(snap.book.getId());
  if(access==='LINK')f.setSharing(DriveApp.Access.ANYONE_WITH_LINK,DriveApp.Permission.VIEW);
  else f.addViewer(access);
}
function intaClientCopy(){return intaRun(()=>{
  const c=intaCfg(),sh=intaPrintable(c);
  const who=intaAsk('Client-only copy','Enter a client email for viewer access, LINK for anyone-with-link view, or leave blank to keep the copy private.');if(who===null)return;
  if(who&&who!=='LINK'&&!intaEmail(who))throw new Error('Enter a valid email, LINK, or leave blank.');
  if(who==='LINK'&&!intaYes('Public link access','Anyone with this new client-only link may view its billing data. Continue?'))return;
  if(who&&who!=='LINK'&&!intaYes('Grant viewer access','Share the separate customer copy with '+who+'?'))return;
  const snap=intaSnapshot(c,sh);
  try{intaShareCopy(snap,who);}catch(e){intaUi().alert('Copy created; sharing not changed',snap.url+'\n'+e.message,intaUi().ButtonSet.OK);return;}
  intaUi().alert('Client-only copy',snap.url+(who?'':'\nPrivate until you grant viewer access.'),intaUi().ButtonSet.OK);
});}
function intaPdf(c,snap){
  SpreadsheetApp.flush();const scale=snap.statement?'2':'4',url='https://docs.google.com/spreadsheets/d/'+snap.book.getId()+'/export?format=pdf&gid='+snap.sheet.getSheetId()+'&size=letter&portrait=true&scale='+scale+'&fitw=true&sheetnames=false&printtitle=false&pagenumbers=false&gridlines=false&fzr=false&top_margin=0.25&bottom_margin=0.25&left_margin=0.25&right_margin=0.25';
  const res=UrlFetchApp.fetch(url,{headers:{Authorization:'Bearer '+ScriptApp.getOAuthToken()},muteHttpExceptions:true});
  if(res.getResponseCode()!==200)throw new Error('PDF export failed (HTTP '+res.getResponseCode()+'). Client-only sheet remains private.');
  const blob=res.getBlob().setName(snap.title+'.pdf');
  let file;const folder=c.kind==='studio'?intaText(c.book.getSheetByName('Settings'),'B26'):'';
  if(folder){const dir=DriveApp.getFolderById(folder);if(dir.getSharingAccess()!==DriveApp.Access.PRIVATE)throw new Error('Archive folder has broad sharing. Use a private folder.');file=dir.createFile(blob);}
  else file=DriveApp.createFile(blob);
  return {file:file,blob:blob,url:file.getUrl()};
}
function intaExportPdf(){return intaRun(()=>{const c=intaCfg(),snap=intaSnapshot(c,intaPrintable(c)),p=intaPdf(c,snap);intaUi().alert('Private PDF created',p.url+'\nClient-only Sheet: '+snap.url,intaUi().ButtonSet.OK);});}
function intaSendEmail(){return intaRun(()=>{
  const c=intaCfg(),sh=intaPrintable(c),to=intaAsk('Send approved document','Enter the exact customer email address.');if(to===null)return;if(!intaEmail(to))throw new Error('Invalid email address.');
  const title=intaTitle(c,sh);if(!intaYes('Confirm email','Send '+title+' as a PDF attachment to '+to+'?'))return;
  const snap=intaSnapshot(c,sh),p=intaPdf(c,snap);
  MailApp.sendEmail({to:to,subject:title,body:'Hello,\n\nPlease find your requested INTA Computer Services document attached.\n\nThank you,\nINTA Computer Services\nhttps://intacomputerservices.com',attachments:[p.blob]});
  intaUi().alert('Sent to '+to+'\nPrivate PDF: '+p.url);
});}
function intaGrow(sh,need){
  const old=sh.getMaxRows(),n=Math.max(100,need),cols=sh.getMaxColumns(),tpl=sh.getRange(old-1,1,1,cols);
  const formulas=tpl.getFormulasR1C1()[0],rules=tpl.getDataValidations()[0],formulaCols={'Register':[8,9,10,18,19],'Payments':[8,9],'Line Archive':[10,12],'Time & Costs':[11,12],'Contracts':[11,12,13,14,15]}[sh.getName()]||[];
  sh.insertRowsBefore(old,n);tpl.copyFormatToRange(sh,1,cols,old,old+n-1);
  sh.getRange(old,1,n,cols).setDataValidations(Array.from({length:n},()=>rules.slice()));
  formulaCols.forEach(c=>{if(formulas[c-1])sh.getRange(old,c,n,1).setFormulasR1C1(Array.from({length:n},()=>[formulas[c-1]]));});
  const book=sh.getParent(),statement=book.getSheetByName('Statement'),analytics=book.getSheetByName('Analytics');
  if(sh.getName()==='Register'&&statement&&statement.getMaxRows()<sh.getMaxRows()+12)statement.insertRowsAfter(statement.getMaxRows(),sh.getMaxRows()+12-statement.getMaxRows());
  if(analytics&&analytics.getMaxRows()<sh.getMaxRows()+40)analytics.insertRowsAfter(analytics.getMaxRows(),sh.getMaxRows()+40-analytics.getMaxRows());
}
function intaFree(sh,n){
  let data=sh.getRange(5,1,sh.getMaxRows()-4,sh.getMaxColumns()).getDisplayValues(),out=[];
  data.forEach((r,i)=>{if(r.every(x=>x===''))out.push(i+5);});
  if(out.length<n){intaGrow(sh,n-out.length);return intaFree(sh,n);}return out.slice(0,n);
}
function intaAllocate(amount,weights){
  const cents=intaCents(amount),sum=weights.reduce((a,b)=>a+Math.max(0,Number(b)),0);
  if(!sum||!cents)return weights.map(()=>0);
  const base=weights.map((w,i)=>{const x=cents*Math.max(0,Number(w))/sum;return {i:i,n:Math.floor(x),f:x-Math.floor(x)};});
  let left=cents-base.reduce((a,b)=>a+b.n,0);base.slice().sort((a,b)=>b.f-a.f||a.i-b.i).forEach(x=>{if(left>0){x.n++;left--;}});
  return base.sort((a,b)=>a.i-b.i).map(x=>x.n/100);
}
function intaArchive(){return intaRun(()=>{
  const c=intaCfg(),sh=c.invoice;intaReady(c,sh);const id=intaText(sh,'G2');
  if(!intaYes('Archive invoice','Save the finalized invoice '+id+' as a private client copy and PDF, and record its immutable billing snapshot?'))return;
  intaLock(()=>{
    const h=intaHistory(c),reg=c.book.getSheetByName('Register');
    if(intaFindKey(h,id,2)||intaFindKey(reg,id,5))throw new Error('Invoice number already archived.');
    const draft=intaCapture(c),snap=intaSnapshot(c,sh),p=intaPdf(c,snap),client=intaText(sh,'F8'),date=intaGet(sh,'C14'),total=Number(intaGet(sh,'H32')),savings=-Number(intaGet(sh,'H29'));
    let regRow=0,lines=[],written=[],historyRow=0;
    try{
      if(c.kind==='studio'){
        const s=c.editor,main=s.getRange('A36:G36').getValues()[0],extra=s.getRange('A53:F53').getValues()[0],tz=c.book.getSpreadsheetTimeZone();
        if(extra[0]&&extra[1] instanceof Date&&intaContractAlreadyBilled(reg,extra[0],extra[1],tz))throw new Error('Contract period already archived.');
        if(main[0]!==id||intaCents(main[5])!==intaCents(total)||!extra[5])throw new Error('Draft changed during archive or client ID is missing.');
        const a=s.getRange('H13:O22').getValues(),costs=s.getRange('E40:E49').getValues().map(r=>Number(r[0]||0)),items=a.map((r,i)=>({r:r,i:i})).filter(x=>x.r[0]&&typeof x.r[5]==='number'),discount=Number(intaGet(c.book.getSheetByName('_Calculation'),'G30')),allocated=intaAllocate(discount,items.map(x=>x.r[5]));
        const archive=c.book.getSheetByName('Line Archive'),lineData=items.map((x,i)=>({a:[id,date,client,x.r[0],x.r[1],x.r[7],x.r[2],x.r[5],allocated[i]],cost:costs[x.i]}));
        const unallocated=Math.round((Number(extra[3]||0)-lineData.reduce((sum,x)=>sum+x.cost,0))*100)/100;
        if(unallocated>0)lineData.push({a:[id,date,client,'Unallocated job costs','Job',1,0,0,0],cost:unallocated});
        regRow=intaFree(reg,1)[0];lines=intaFree(archive,lineData.length);
        reg.getRange(regRow,1,1,7).setValues([main]);reg.getRange(regRow,11).setValue(p.url);reg.getRange(regRow,13,1,5).setValues([extra.slice(0,5)]);reg.getRange(regRow,20).setValue(snap.url);reg.getRange(regRow,21).setValue(extra[5]);
        lineData.forEach((x,i)=>{const r=lines[i];written.push(r);archive.getRange(r,1,1,9).setValues([x.a]);archive.getRange(r,11).setValue(x.cost);});
      }
      historyRow=h.getLastRow()+1;h.getRange(historyRow,1,1,8).setValues([[id,date,client,JSON.stringify(draft),snap.url,p.url,total,savings]]);SpreadsheetApp.flush();
    }catch(e){
      if(regRow){reg.getRange(regRow,1,1,7).clearContent();reg.getRange(regRow,11).clearContent();reg.getRange(regRow,13,1,5).clearContent();reg.getRange(regRow,20,1,2).clearContent();}
      if(c.kind==='studio'){const l=c.book.getSheetByName('Line Archive');written.forEach(r=>{l.getRange(r,1,1,9).clearContent();l.getRange(r,11).clearContent();});}
      if(historyRow)h.getRange(historyRow,1,1,8).clearContent();throw new Error('Archive record not completed. Private output files were retained: '+snap.url+' / '+p.url+' — '+e.message);
    }
    intaUi().alert('Archived '+id,'Client copy (private): '+snap.url+'\nPDF: '+p.url,intaUi().ButtonSet.OK);
  });
});}
function intaPayment(){return intaRun(()=>{
  const c=intaCfg(),defaultId=intaText(c.editor,c.id);
  const id=c.kind==='studio'?intaAsk('Invoice number','Enter registered or current invoice ID: '+defaultId):defaultId;
  if(id===null)return;if(!id)throw new Error('Enter an invoice number first.');
  const raw=intaAsk('Payment or refund','Enter PAYMENT or REFUND.');if(raw===null)return;
  const t=raw.toUpperCase();if(t!=='PAYMENT'&&t!=='REFUND')throw new Error('Choose PAYMENT or REFUND.');
  const amt=intaAsk(t,'Enter confirmed amount.');if(amt===null)return;const amount=intaPositive(amt);
  const method=intaAsk('Method','Enter ACH, Check, Card, Cash, QuickBooks, or Other.');if(method===null)return;
  const ref=intaAsk('Reference','Optional transaction reference.');if(ref===null)return;
  if(!intaYes('Record '+t.toLowerCase(),id+' — '+intaMoney(amount)+' via '+method+'?'))return;
  intaLock(()=>{
    if(c.kind==='quick'){
      const prev=Number(intaGet(c.editor,'G13')||0),next=prev+(t==='REFUND'?-amount:amount);if(next<0)throw new Error('Refund exceeds recorded payments.');intaSet(c.editor,'G13',Math.round(next*100)/100);
    }else{
      const reg=c.book.getSheetByName('Register'),p=c.book.getSheetByName('Payments');if(!intaFindKey(reg,id,5)&&id!==defaultId)throw new Error('Unknown invoice ID.');
      const r=intaFree(p,1)[0];p.getRange(r,1,1,7).setValues([[id,new Date(),amount,t==='REFUND'?'Refund':'Payment',method,ref,'Recorded manually']]);
    }
  });
});}
function intaReminder(){return intaRun(()=>{
  const c=intaCfg();if(c.kind!=='studio')throw new Error('Reminders use the Studio Register.');
  const reg=c.book.getSheetByName('Register'),id=intaAsk('Overdue reminder','Enter the registered invoice number.');if(id===null)return;
  const r=intaFindKey(reg,id,5);if(!r)throw new Error('Unknown invoice.');
  const v=reg.getRange(r,1,1,21).getValues()[0];if(v[9]!=='OVERDUE'||Number(v[8])<=0)throw new Error('Invoice is not overdue.');
  const clients=c.book.getSheetByName('Clients'),cr=intaFindKey(clients,v[20],5);if(!cr)throw new Error('Register client ID is missing or invalid.');
  const to=intaText(clients,'E'+cr);if(!intaEmail(to))throw new Error('Client email is missing.');
  if(!intaYes('Create reminder draft','Create a Gmail draft (not send) to '+to+' for '+intaMoney(v[8])+'?'))return;
  GmailApp.createDraft(to,'INTA invoice '+id+' — balance reminder','Hello,\n\nThis is a friendly reminder that invoice '+id+' has an outstanding balance of '+intaMoney(v[8])+'. Please let us know if you have any questions.\n\nThank you,\nINTA Computer Services');
  intaUi().alert('Reminder draft created. Review it in Gmail before sending.');
});}
