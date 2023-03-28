#[doc = "Register `FDCAN_TTIR` reader"]
pub struct R(crate::R<FDCAN_TTIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTIR` writer"]
pub struct W(crate::W<FDCAN_TTIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTIR_SPEC>;
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
impl From<crate::W<FDCAN_TTIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBC` reader - SBC"]
pub type SBC_R = crate::BitReader<bool>;
#[doc = "Field `SBC` writer - SBC"]
pub type SBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `SMC` reader - SMC"]
pub type SMC_R = crate::BitReader<bool>;
#[doc = "Field `SMC` writer - SMC"]
pub type SMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `CSM` reader - CSM"]
pub type CSM_R = crate::BitReader<bool>;
#[doc = "Field `CSM` writer - CSM"]
pub type CSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `SOG` reader - SOG"]
pub type SOG_R = crate::BitReader<bool>;
#[doc = "Field `SOG` writer - SOG"]
pub type SOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `RTMI` reader - RTMI"]
pub type RTMI_R = crate::BitReader<bool>;
#[doc = "Field `RTMI` writer - RTMI"]
pub type RTMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `TTMI` reader - TTMI"]
pub type TTMI_R = crate::BitReader<bool>;
#[doc = "Field `TTMI` writer - TTMI"]
pub type TTMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `SWE` reader - SWE"]
pub type SWE_R = crate::BitReader<bool>;
#[doc = "Field `SWE` writer - SWE"]
pub type SWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `GTW` reader - GTW"]
pub type GTW_R = crate::BitReader<bool>;
#[doc = "Field `GTW` writer - GTW"]
pub type GTW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `GTD` reader - GTD"]
pub type GTD_R = crate::BitReader<bool>;
#[doc = "Field `GTD` writer - GTD"]
pub type GTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `GTE` reader - GTE"]
pub type GTE_R = crate::BitReader<bool>;
#[doc = "Field `GTE` writer - GTE"]
pub type GTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `TXU` reader - TXU"]
pub type TXU_R = crate::BitReader<bool>;
#[doc = "Field `TXU` writer - TXU"]
pub type TXU_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `TXO` reader - TXO"]
pub type TXO_R = crate::BitReader<bool>;
#[doc = "Field `TXO` writer - TXO"]
pub type TXO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `SE1` reader - SE1"]
pub type SE1_R = crate::BitReader<bool>;
#[doc = "Field `SE1` writer - SE1"]
pub type SE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `SE2` reader - SE2"]
pub type SE2_R = crate::BitReader<bool>;
#[doc = "Field `SE2` writer - SE2"]
pub type SE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `ELC` reader - ELC"]
pub type ELC_R = crate::BitReader<bool>;
#[doc = "Field `ELC` writer - ELC"]
pub type ELC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `IWTG` reader - IWTG"]
pub type IWTG_R = crate::BitReader<bool>;
#[doc = "Field `IWTG` writer - IWTG"]
pub type IWTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `WT` reader - WT"]
pub type WT_R = crate::BitReader<bool>;
#[doc = "Field `WT` writer - WT"]
pub type WT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `AW` reader - AW"]
pub type AW_R = crate::BitReader<bool>;
#[doc = "Field `AW` writer - AW"]
pub type AW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
#[doc = "Field `CER` reader - CER"]
pub type CER_R = crate::BitReader<bool>;
#[doc = "Field `CER` writer - CER"]
pub type CER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SBC"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMC"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSM"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOG"]
    #[inline(always)]
    pub fn sog(&self) -> SOG_R {
        SOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTMI"]
    #[inline(always)]
    pub fn rtmi(&self) -> RTMI_R {
        RTMI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TTMI"]
    #[inline(always)]
    pub fn ttmi(&self) -> TTMI_R {
        TTMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWE"]
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTW"]
    #[inline(always)]
    pub fn gtw(&self) -> GTW_R {
        GTW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTD"]
    #[inline(always)]
    pub fn gtd(&self) -> GTD_R {
        GTD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTE"]
    #[inline(always)]
    pub fn gte(&self) -> GTE_R {
        GTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXU"]
    #[inline(always)]
    pub fn txu(&self) -> TXU_R {
        TXU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXO"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SE1"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SE2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ELC"]
    #[inline(always)]
    pub fn elc(&self) -> ELC_R {
        ELC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IWTG"]
    #[inline(always)]
    pub fn iwtg(&self) -> IWTG_R {
        IWTG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AW"]
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CER"]
    #[inline(always)]
    pub fn cer(&self) -> CER_R {
        CER_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBC"]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<0> {
        SBC_W::new(self)
    }
    #[doc = "Bit 1 - SMC"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SMC_W<1> {
        SMC_W::new(self)
    }
    #[doc = "Bit 2 - CSM"]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CSM_W<2> {
        CSM_W::new(self)
    }
    #[doc = "Bit 3 - SOG"]
    #[inline(always)]
    #[must_use]
    pub fn sog(&mut self) -> SOG_W<3> {
        SOG_W::new(self)
    }
    #[doc = "Bit 4 - RTMI"]
    #[inline(always)]
    #[must_use]
    pub fn rtmi(&mut self) -> RTMI_W<4> {
        RTMI_W::new(self)
    }
    #[doc = "Bit 5 - TTMI"]
    #[inline(always)]
    #[must_use]
    pub fn ttmi(&mut self) -> TTMI_W<5> {
        TTMI_W::new(self)
    }
    #[doc = "Bit 6 - SWE"]
    #[inline(always)]
    #[must_use]
    pub fn swe(&mut self) -> SWE_W<6> {
        SWE_W::new(self)
    }
    #[doc = "Bit 7 - GTW"]
    #[inline(always)]
    #[must_use]
    pub fn gtw(&mut self) -> GTW_W<7> {
        GTW_W::new(self)
    }
    #[doc = "Bit 8 - GTD"]
    #[inline(always)]
    #[must_use]
    pub fn gtd(&mut self) -> GTD_W<8> {
        GTD_W::new(self)
    }
    #[doc = "Bit 9 - GTE"]
    #[inline(always)]
    #[must_use]
    pub fn gte(&mut self) -> GTE_W<9> {
        GTE_W::new(self)
    }
    #[doc = "Bit 10 - TXU"]
    #[inline(always)]
    #[must_use]
    pub fn txu(&mut self) -> TXU_W<10> {
        TXU_W::new(self)
    }
    #[doc = "Bit 11 - TXO"]
    #[inline(always)]
    #[must_use]
    pub fn txo(&mut self) -> TXO_W<11> {
        TXO_W::new(self)
    }
    #[doc = "Bit 12 - SE1"]
    #[inline(always)]
    #[must_use]
    pub fn se1(&mut self) -> SE1_W<12> {
        SE1_W::new(self)
    }
    #[doc = "Bit 13 - SE2"]
    #[inline(always)]
    #[must_use]
    pub fn se2(&mut self) -> SE2_W<13> {
        SE2_W::new(self)
    }
    #[doc = "Bit 14 - ELC"]
    #[inline(always)]
    #[must_use]
    pub fn elc(&mut self) -> ELC_W<14> {
        ELC_W::new(self)
    }
    #[doc = "Bit 15 - IWTG"]
    #[inline(always)]
    #[must_use]
    pub fn iwtg(&mut self) -> IWTG_W<15> {
        IWTG_W::new(self)
    }
    #[doc = "Bit 16 - WT"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<16> {
        WT_W::new(self)
    }
    #[doc = "Bit 17 - AW"]
    #[inline(always)]
    #[must_use]
    pub fn aw(&mut self) -> AW_W<17> {
        AW_W::new(self)
    }
    #[doc = "Bit 18 - CER"]
    #[inline(always)]
    #[must_use]
    pub fn cer(&mut self) -> CER_W<18> {
        CER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttir](index.html) module"]
pub struct FDCAN_TTIR_SPEC;
impl crate::RegisterSpec for FDCAN_TTIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttir::R](R) reader structure"]
impl crate::Readable for FDCAN_TTIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttir::W](W) writer structure"]
impl crate::Writable for FDCAN_TTIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTIR to value 0"]
impl crate::Resettable for FDCAN_TTIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
