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
#[doc = "Field `RF0N` reader - RF0N"]
pub type RF0N_R = crate::BitReader<bool>;
#[doc = "Field `RF0N` writer - RF0N"]
pub type RF0N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF0F` reader - RF0F"]
pub type RF0F_R = crate::BitReader<bool>;
#[doc = "Field `RF0F` writer - RF0F"]
pub type RF0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF0L` reader - RF0L"]
pub type RF0L_R = crate::BitReader<bool>;
#[doc = "Field `RF0L` writer - RF0L"]
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF1N` reader - RF1N"]
pub type RF1N_R = crate::BitReader<bool>;
#[doc = "Field `RF1N` writer - RF1N"]
pub type RF1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF1F` reader - RF1F"]
pub type RF1F_R = crate::BitReader<bool>;
#[doc = "Field `RF1F` writer - RF1F"]
pub type RF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `RF1L` reader - RF1L"]
pub type RF1L_R = crate::BitReader<bool>;
#[doc = "Field `RF1L` writer - RF1L"]
pub type RF1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `HPM` reader - HPM"]
pub type HPM_R = crate::BitReader<bool>;
#[doc = "Field `HPM` writer - HPM"]
pub type HPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TC` reader - TC"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - TC"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TCF` reader - TCF"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - TCF"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TFE` reader - TFE"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - TFE"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TEFN` reader - TEFN"]
pub type TEFN_R = crate::BitReader<bool>;
#[doc = "Field `TEFN` writer - TEFN"]
pub type TEFN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TEFF` reader - TEFF"]
pub type TEFF_R = crate::BitReader<bool>;
#[doc = "Field `TEFF` writer - TEFF"]
pub type TEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TEFL` reader - TEFL"]
pub type TEFL_R = crate::BitReader<bool>;
#[doc = "Field `TEFL` writer - TEFL"]
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TSW` reader - TSW"]
pub type TSW_R = crate::BitReader<bool>;
#[doc = "Field `TSW` writer - TSW"]
pub type TSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `MRAF` reader - MRAF"]
pub type MRAF_R = crate::BitReader<bool>;
#[doc = "Field `MRAF` writer - MRAF"]
pub type MRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `TOO` reader - TOO"]
pub type TOO_R = crate::BitReader<bool>;
#[doc = "Field `TOO` writer - TOO"]
pub type TOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `ELO` reader - ELO"]
pub type ELO_R = crate::BitReader<bool>;
#[doc = "Field `ELO` writer - ELO"]
pub type ELO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `EP` reader - EP"]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EP` writer - EP"]
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `EW` reader - EW"]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `EW` writer - EW"]
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `BO` reader - BO"]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `BO` writer - BO"]
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `WDI` reader - WDI"]
pub type WDI_R = crate::BitReader<bool>;
#[doc = "Field `WDI` writer - WDI"]
pub type WDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `PEA` reader - PEA"]
pub type PEA_R = crate::BitReader<bool>;
#[doc = "Field `PEA` writer - PEA"]
pub type PEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `PED` reader - PED"]
pub type PED_R = crate::BitReader<bool>;
#[doc = "Field `PED` writer - PED"]
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
#[doc = "Field `ARA` reader - ARA"]
pub type ARA_R = crate::BitReader<bool>;
#[doc = "Field `ARA` writer - ARA"]
pub type ARA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RF0N"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RF0F"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RF1N"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RF1F"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RF1L"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HPM"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TEFN"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TEFF"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSW"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MRAF"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TOO"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELO"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WDI"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PEA"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PED"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ARA"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RF0N"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<0> {
        RF0N_W::new(self)
    }
    #[doc = "Bit 1 - RF0F"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<1> {
        RF0F_W::new(self)
    }
    #[doc = "Bit 2 - RF0L"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<2> {
        RF0L_W::new(self)
    }
    #[doc = "Bit 3 - RF1N"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<3> {
        RF1N_W::new(self)
    }
    #[doc = "Bit 4 - RF1F"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<4> {
        RF1F_W::new(self)
    }
    #[doc = "Bit 5 - RF1L"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<5> {
        RF1L_W::new(self)
    }
    #[doc = "Bit 6 - HPM"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<6> {
        HPM_W::new(self)
    }
    #[doc = "Bit 7 - TC"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<7> {
        TC_W::new(self)
    }
    #[doc = "Bit 8 - TCF"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<8> {
        TCF_W::new(self)
    }
    #[doc = "Bit 9 - TFE"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<9> {
        TFE_W::new(self)
    }
    #[doc = "Bit 10 - TEFN"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<10> {
        TEFN_W::new(self)
    }
    #[doc = "Bit 11 - TEFF"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<11> {
        TEFF_W::new(self)
    }
    #[doc = "Bit 12 - TEFL"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<12> {
        TEFL_W::new(self)
    }
    #[doc = "Bit 13 - TSW"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<13> {
        TSW_W::new(self)
    }
    #[doc = "Bit 14 - MRAF"]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<14> {
        MRAF_W::new(self)
    }
    #[doc = "Bit 15 - TOO"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<15> {
        TOO_W::new(self)
    }
    #[doc = "Bit 16 - ELO"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<16> {
        ELO_W::new(self)
    }
    #[doc = "Bit 17 - EP"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<17> {
        EP_W::new(self)
    }
    #[doc = "Bit 18 - EW"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<18> {
        EW_W::new(self)
    }
    #[doc = "Bit 19 - BO"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<19> {
        BO_W::new(self)
    }
    #[doc = "Bit 20 - WDI"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<20> {
        WDI_W::new(self)
    }
    #[doc = "Bit 21 - PEA"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<21> {
        PEA_W::new(self)
    }
    #[doc = "Bit 22 - PED"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<22> {
        PED_W::new(self)
    }
    #[doc = "Bit 23 - ARA"]
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
#[doc = "FDCAN Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ir](index.html) module"]
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
