#[doc = "Register `FDCAN_IR` reader"]
pub struct R(crate::R<FDCAN_IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_IR` writer"]
pub struct W(crate::W<FDCAN_IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FDCAN_IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0N` reader - Rx FIFO 0 new message"]
pub type RF0N_R = crate::BitReader<bool>;
#[doc = "Field `RF0N` writer - Rx FIFO 0 new message"]
pub type RF0N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF0F` reader - Rx FIFO 0 full"]
pub type RF0F_R = crate::BitReader<bool>;
#[doc = "Field `RF0F` writer - Rx FIFO 0 full"]
pub type RF0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost"]
pub type RF0L_R = crate::BitReader<bool>;
#[doc = "Field `RF0L` writer - Rx FIFO 0 message lost"]
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF1N` reader - Rx FIFO 1 new message"]
pub type RF1N_R = crate::BitReader<bool>;
#[doc = "Field `RF1N` writer - Rx FIFO 1 new message"]
pub type RF1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF1F` reader - Rx FIFO 1 full"]
pub type RF1F_R = crate::BitReader<bool>;
#[doc = "Field `RF1F` writer - Rx FIFO 1 full"]
pub type RF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost"]
pub type RF1L_R = crate::BitReader<bool>;
#[doc = "Field `RF1L` writer - Rx FIFO 1 message lost"]
pub type RF1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `HPM` reader - High-priority message"]
pub type HPM_R = crate::BitReader<bool>;
#[doc = "Field `HPM` writer - High-priority message"]
pub type HPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TC` reader - Transmission completed"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - Transmission completed"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TCF` reader - Transmission cancellation finished"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - Transmission cancellation finished"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Tx FIFO empty"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Tx FIFO empty"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TEFN` reader - Tx event FIFO New Entry"]
pub type TEFN_R = crate::BitReader<bool>;
#[doc = "Field `TEFN` writer - Tx event FIFO New Entry"]
pub type TEFN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TEFF` reader - Tx event FIFO full"]
pub type TEFF_R = crate::BitReader<bool>;
#[doc = "Field `TEFF` writer - Tx event FIFO full"]
pub type TEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TEFL` reader - Tx event FIFO element lost"]
pub type TEFL_R = crate::BitReader<bool>;
#[doc = "Field `TEFL` writer - Tx event FIFO element lost"]
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TSW` reader - Timestamp wraparound"]
pub type TSW_R = crate::BitReader<bool>;
#[doc = "Field `TSW` writer - Timestamp wraparound"]
pub type TSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `MRAF` reader - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MRAF_R = crate::BitReader<bool>;
#[doc = "Field `MRAF` writer - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TOO` reader - Timeout occurred"]
pub type TOO_R = crate::BitReader<bool>;
#[doc = "Field `TOO` writer - Timeout occurred"]
pub type TOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `ELO` reader - Error logging overflow"]
pub type ELO_R = crate::BitReader<bool>;
#[doc = "Field `ELO` writer - Error logging overflow"]
pub type ELO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `EP` reader - Error passive"]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EP` writer - Error passive"]
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `EW` reader - Warning status"]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `EW` writer - Warning status"]
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `BO` reader - Bus_Off status"]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `BO` writer - Bus_Off status"]
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `WDI` reader - Watchdog interrupt"]
pub type WDI_R = crate::BitReader<bool>;
#[doc = "Field `WDI` writer - Watchdog interrupt"]
pub type WDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `PEA` reader - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PEA_R = crate::BitReader<bool>;
#[doc = "Field `PEA` writer - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `PED` reader - Protocol error in data phase (data bit time is used)"]
pub type PED_R = crate::BitReader<bool>;
#[doc = "Field `PED` writer - Protocol error in data phase (data bit time is used)"]
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `ARA` reader - Access to reserved address"]
pub type ARA_R = crate::BitReader<bool>;
#[doc = "Field `ARA` writer - Access to reserved address"]
pub type ARA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<0> {
        RF0N_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<1> {
        RF0F_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<2> {
        RF0L_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<3> {
        RF1N_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<4> {
        RF1F_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<5> {
        RF1L_W::new(self)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<6> {
        HPM_W::new(self)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<7> {
        TC_W::new(self)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<8> {
        TCF_W::new(self)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<9> {
        TFE_W::new(self)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<10> {
        TEFN_W::new(self)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<11> {
        TEFF_W::new(self)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<12> {
        TEFL_W::new(self)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<13> {
        TSW_W::new(self)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<14> {
        MRAF_W::new(self)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<15> {
        TOO_W::new(self)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<16> {
        ELO_W::new(self)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<17> {
        EP_W::new(self)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<18> {
        EW_W::new(self)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<19> {
        BO_W::new(self)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<20> {
        WDI_W::new(self)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<21> {
        PEA_W::new(self)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<22> {
        PED_W::new(self)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> ARA_W<23> {
        ARA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ir](index.html) module"]
pub struct FDCAN_IR_SPEC;
impl crate::RegisterSpec for FDCAN_IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ir::R](R) reader structure"]
impl crate::Readable for FDCAN_IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ir::W](W) writer structure"]
impl crate::Writable for FDCAN_IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_IR to value 0"]
impl crate::Resettable for FDCAN_IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
