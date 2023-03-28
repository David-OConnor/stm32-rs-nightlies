#[doc = "Register `FDCAN_ILS` reader"]
pub struct R(crate::R<FDCAN_ILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_ILS` writer"]
pub struct W(crate::W<FDCAN_ILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_ILS_SPEC>;
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
impl From<crate::W<FDCAN_ILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_ILS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0NL` reader - RF0NL"]
pub type RF0NL_R = crate::BitReader<bool>;
#[doc = "Field `RF0NL` writer - RF0NL"]
pub type RF0NL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `RF0WL` reader - RF0WL"]
pub type RF0WL_R = crate::BitReader<bool>;
#[doc = "Field `RF0WL` writer - RF0WL"]
pub type RF0WL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `RF0FL` reader - RF0FL"]
pub type RF0FL_R = crate::BitReader<bool>;
#[doc = "Field `RF0FL` writer - RF0FL"]
pub type RF0FL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `RF0LL` reader - RF0LL"]
pub type RF0LL_R = crate::BitReader<bool>;
#[doc = "Field `RF0LL` writer - RF0LL"]
pub type RF0LL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `RF1NL` reader - RF1NL"]
pub type RF1NL_R = crate::BitReader<bool>;
#[doc = "Field `RF1NL` writer - RF1NL"]
pub type RF1NL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `RF1WL` reader - RF1WL"]
pub type RF1WL_R = crate::BitReader<bool>;
#[doc = "Field `RF1WL` writer - RF1WL"]
pub type RF1WL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `RF1FL` reader - RF1FL"]
pub type RF1FL_R = crate::BitReader<bool>;
#[doc = "Field `RF1FL` writer - RF1FL"]
pub type RF1FL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `RF1LL` reader - RF1LL"]
pub type RF1LL_R = crate::BitReader<bool>;
#[doc = "Field `RF1LL` writer - RF1LL"]
pub type RF1LL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `HPML` reader - HPML"]
pub type HPML_R = crate::BitReader<bool>;
#[doc = "Field `HPML` writer - HPML"]
pub type HPML_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TCL` reader - TCL"]
pub type TCL_R = crate::BitReader<bool>;
#[doc = "Field `TCL` writer - TCL"]
pub type TCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TCFL` reader - TCFL"]
pub type TCFL_R = crate::BitReader<bool>;
#[doc = "Field `TCFL` writer - TCFL"]
pub type TCFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TFEL` reader - TFEL"]
pub type TFEL_R = crate::BitReader<bool>;
#[doc = "Field `TFEL` writer - TFEL"]
pub type TFEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TEFNL` reader - TEFNL"]
pub type TEFNL_R = crate::BitReader<bool>;
#[doc = "Field `TEFNL` writer - TEFNL"]
pub type TEFNL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TEFWL` reader - TEFWL"]
pub type TEFWL_R = crate::BitReader<bool>;
#[doc = "Field `TEFWL` writer - TEFWL"]
pub type TEFWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TEFFL` reader - TEFFL"]
pub type TEFFL_R = crate::BitReader<bool>;
#[doc = "Field `TEFFL` writer - TEFFL"]
pub type TEFFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TEFLL` reader - TEFLL"]
pub type TEFLL_R = crate::BitReader<bool>;
#[doc = "Field `TEFLL` writer - TEFLL"]
pub type TEFLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TSWL` reader - TSWL"]
pub type TSWL_R = crate::BitReader<bool>;
#[doc = "Field `TSWL` writer - TSWL"]
pub type TSWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `MRAFL` reader - MRAFL"]
pub type MRAFL_R = crate::BitReader<bool>;
#[doc = "Field `MRAFL` writer - MRAFL"]
pub type MRAFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `TOOL` reader - TOOL"]
pub type TOOL_R = crate::BitReader<bool>;
#[doc = "Field `TOOL` writer - TOOL"]
pub type TOOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `DRXL` reader - DRXL"]
pub type DRXL_R = crate::BitReader<bool>;
#[doc = "Field `DRXL` writer - DRXL"]
pub type DRXL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `BECL` reader - BECL"]
pub type BECL_R = crate::BitReader<bool>;
#[doc = "Field `BECL` writer - BECL"]
pub type BECL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `BEUL` reader - BEUL"]
pub type BEUL_R = crate::BitReader<bool>;
#[doc = "Field `BEUL` writer - BEUL"]
pub type BEUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `ELOL` reader - ELOL"]
pub type ELOL_R = crate::BitReader<bool>;
#[doc = "Field `ELOL` writer - ELOL"]
pub type ELOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `EPL` reader - EPL"]
pub type EPL_R = crate::BitReader<bool>;
#[doc = "Field `EPL` writer - EPL"]
pub type EPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `EWL` reader - EWL"]
pub type EWL_R = crate::BitReader<bool>;
#[doc = "Field `EWL` writer - EWL"]
pub type EWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `BOL` reader - BOL"]
pub type BOL_R = crate::BitReader<bool>;
#[doc = "Field `BOL` writer - BOL"]
pub type BOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `WDIL` reader - WDIL"]
pub type WDIL_R = crate::BitReader<bool>;
#[doc = "Field `WDIL` writer - WDIL"]
pub type WDIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `PEAL` reader - PEAL"]
pub type PEAL_R = crate::BitReader<bool>;
#[doc = "Field `PEAL` writer - PEAL"]
pub type PEAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `PEDL` reader - PEDL"]
pub type PEDL_R = crate::BitReader<bool>;
#[doc = "Field `PEDL` writer - PEDL"]
pub type PEDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
#[doc = "Field `ARAL` reader - ARAL"]
pub type ARAL_R = crate::BitReader<bool>;
#[doc = "Field `ARAL` writer - ARAL"]
pub type ARAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ILS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RF0NL"]
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RF0WL"]
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RF0FL"]
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RF0LL"]
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RF1NL"]
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RF1WL"]
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RF1FL"]
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RF1LL"]
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HPML"]
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TCL"]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TCFL"]
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TFEL"]
    #[inline(always)]
    pub fn tfel(&self) -> TFEL_R {
        TFEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TEFNL"]
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TEFWL"]
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEFFL"]
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TEFLL"]
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TSWL"]
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MRAFL"]
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TOOL"]
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DRXL"]
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BECL"]
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEUL"]
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELOL"]
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EPL"]
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EWL"]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BOL"]
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - WDIL"]
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PEAL"]
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PEDL"]
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ARAL"]
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RF0NL"]
    #[inline(always)]
    #[must_use]
    pub fn rf0nl(&mut self) -> RF0NL_W<0> {
        RF0NL_W::new(self)
    }
    #[doc = "Bit 1 - RF0WL"]
    #[inline(always)]
    #[must_use]
    pub fn rf0wl(&mut self) -> RF0WL_W<1> {
        RF0WL_W::new(self)
    }
    #[doc = "Bit 2 - RF0FL"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fl(&mut self) -> RF0FL_W<2> {
        RF0FL_W::new(self)
    }
    #[doc = "Bit 3 - RF0LL"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ll(&mut self) -> RF0LL_W<3> {
        RF0LL_W::new(self)
    }
    #[doc = "Bit 4 - RF1NL"]
    #[inline(always)]
    #[must_use]
    pub fn rf1nl(&mut self) -> RF1NL_W<4> {
        RF1NL_W::new(self)
    }
    #[doc = "Bit 5 - RF1WL"]
    #[inline(always)]
    #[must_use]
    pub fn rf1wl(&mut self) -> RF1WL_W<5> {
        RF1WL_W::new(self)
    }
    #[doc = "Bit 6 - RF1FL"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fl(&mut self) -> RF1FL_W<6> {
        RF1FL_W::new(self)
    }
    #[doc = "Bit 7 - RF1LL"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ll(&mut self) -> RF1LL_W<7> {
        RF1LL_W::new(self)
    }
    #[doc = "Bit 8 - HPML"]
    #[inline(always)]
    #[must_use]
    pub fn hpml(&mut self) -> HPML_W<8> {
        HPML_W::new(self)
    }
    #[doc = "Bit 9 - TCL"]
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TCL_W<9> {
        TCL_W::new(self)
    }
    #[doc = "Bit 10 - TCFL"]
    #[inline(always)]
    #[must_use]
    pub fn tcfl(&mut self) -> TCFL_W<10> {
        TCFL_W::new(self)
    }
    #[doc = "Bit 11 - TFEL"]
    #[inline(always)]
    #[must_use]
    pub fn tfel(&mut self) -> TFEL_W<11> {
        TFEL_W::new(self)
    }
    #[doc = "Bit 12 - TEFNL"]
    #[inline(always)]
    #[must_use]
    pub fn tefnl(&mut self) -> TEFNL_W<12> {
        TEFNL_W::new(self)
    }
    #[doc = "Bit 13 - TEFWL"]
    #[inline(always)]
    #[must_use]
    pub fn tefwl(&mut self) -> TEFWL_W<13> {
        TEFWL_W::new(self)
    }
    #[doc = "Bit 14 - TEFFL"]
    #[inline(always)]
    #[must_use]
    pub fn teffl(&mut self) -> TEFFL_W<14> {
        TEFFL_W::new(self)
    }
    #[doc = "Bit 15 - TEFLL"]
    #[inline(always)]
    #[must_use]
    pub fn tefll(&mut self) -> TEFLL_W<15> {
        TEFLL_W::new(self)
    }
    #[doc = "Bit 16 - TSWL"]
    #[inline(always)]
    #[must_use]
    pub fn tswl(&mut self) -> TSWL_W<16> {
        TSWL_W::new(self)
    }
    #[doc = "Bit 17 - MRAFL"]
    #[inline(always)]
    #[must_use]
    pub fn mrafl(&mut self) -> MRAFL_W<17> {
        MRAFL_W::new(self)
    }
    #[doc = "Bit 18 - TOOL"]
    #[inline(always)]
    #[must_use]
    pub fn tool(&mut self) -> TOOL_W<18> {
        TOOL_W::new(self)
    }
    #[doc = "Bit 19 - DRXL"]
    #[inline(always)]
    #[must_use]
    pub fn drxl(&mut self) -> DRXL_W<19> {
        DRXL_W::new(self)
    }
    #[doc = "Bit 20 - BECL"]
    #[inline(always)]
    #[must_use]
    pub fn becl(&mut self) -> BECL_W<20> {
        BECL_W::new(self)
    }
    #[doc = "Bit 21 - BEUL"]
    #[inline(always)]
    #[must_use]
    pub fn beul(&mut self) -> BEUL_W<21> {
        BEUL_W::new(self)
    }
    #[doc = "Bit 22 - ELOL"]
    #[inline(always)]
    #[must_use]
    pub fn elol(&mut self) -> ELOL_W<22> {
        ELOL_W::new(self)
    }
    #[doc = "Bit 23 - EPL"]
    #[inline(always)]
    #[must_use]
    pub fn epl(&mut self) -> EPL_W<23> {
        EPL_W::new(self)
    }
    #[doc = "Bit 24 - EWL"]
    #[inline(always)]
    #[must_use]
    pub fn ewl(&mut self) -> EWL_W<24> {
        EWL_W::new(self)
    }
    #[doc = "Bit 25 - BOL"]
    #[inline(always)]
    #[must_use]
    pub fn bol(&mut self) -> BOL_W<25> {
        BOL_W::new(self)
    }
    #[doc = "Bit 26 - WDIL"]
    #[inline(always)]
    #[must_use]
    pub fn wdil(&mut self) -> WDIL_W<26> {
        WDIL_W::new(self)
    }
    #[doc = "Bit 27 - PEAL"]
    #[inline(always)]
    #[must_use]
    pub fn peal(&mut self) -> PEAL_W<27> {
        PEAL_W::new(self)
    }
    #[doc = "Bit 28 - PEDL"]
    #[inline(always)]
    #[must_use]
    pub fn pedl(&mut self) -> PEDL_W<28> {
        PEDL_W::new(self)
    }
    #[doc = "Bit 29 - ARAL"]
    #[inline(always)]
    #[must_use]
    pub fn aral(&mut self) -> ARAL_W<29> {
        ARAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ils](index.html) module"]
pub struct FDCAN_ILS_SPEC;
impl crate::RegisterSpec for FDCAN_ILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ils::R](R) reader structure"]
impl crate::Readable for FDCAN_ILS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ils::W](W) writer structure"]
impl crate::Writable for FDCAN_ILS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_ILS to value 0"]
impl crate::Resettable for FDCAN_ILS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
