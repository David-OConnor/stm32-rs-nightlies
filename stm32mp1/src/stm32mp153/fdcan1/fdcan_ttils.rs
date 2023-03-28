#[doc = "Register `FDCAN_TTILS` reader"]
pub struct R(crate::R<FDCAN_TTILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTILS` writer"]
pub struct W(crate::W<FDCAN_TTILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTILS_SPEC>;
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
impl From<crate::W<FDCAN_TTILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTILS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBCL` reader - SBCL"]
pub type SBCL_R = crate::BitReader<bool>;
#[doc = "Field `SBCL` writer - SBCL"]
pub type SBCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `SMCL` reader - SMCL"]
pub type SMCL_R = crate::BitReader<bool>;
#[doc = "Field `SMCL` writer - SMCL"]
pub type SMCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `CSML` reader - CSML"]
pub type CSML_R = crate::BitReader<bool>;
#[doc = "Field `CSML` writer - CSML"]
pub type CSML_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `SOGL` reader - SOGL"]
pub type SOGL_R = crate::BitReader<bool>;
#[doc = "Field `SOGL` writer - SOGL"]
pub type SOGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `RTMIL` reader - RTMIL"]
pub type RTMIL_R = crate::BitReader<bool>;
#[doc = "Field `RTMIL` writer - RTMIL"]
pub type RTMIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `TTMIL` reader - TTMIL"]
pub type TTMIL_R = crate::BitReader<bool>;
#[doc = "Field `TTMIL` writer - TTMIL"]
pub type TTMIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `SWEL` reader - SWEL"]
pub type SWEL_R = crate::BitReader<bool>;
#[doc = "Field `SWEL` writer - SWEL"]
pub type SWEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `GTWL` reader - GTWL"]
pub type GTWL_R = crate::BitReader<bool>;
#[doc = "Field `GTWL` writer - GTWL"]
pub type GTWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `GTDL` reader - GTDL"]
pub type GTDL_R = crate::BitReader<bool>;
#[doc = "Field `GTDL` writer - GTDL"]
pub type GTDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `GTEL` reader - GTEL"]
pub type GTEL_R = crate::BitReader<bool>;
#[doc = "Field `GTEL` writer - GTEL"]
pub type GTEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `TXUL` reader - TXUL"]
pub type TXUL_R = crate::BitReader<bool>;
#[doc = "Field `TXUL` writer - TXUL"]
pub type TXUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `TXOL` reader - TXOL"]
pub type TXOL_R = crate::BitReader<bool>;
#[doc = "Field `TXOL` writer - TXOL"]
pub type TXOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `SE1L` reader - SE1L"]
pub type SE1L_R = crate::BitReader<bool>;
#[doc = "Field `SE1L` writer - SE1L"]
pub type SE1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `SE2L` reader - SE2L"]
pub type SE2L_R = crate::BitReader<bool>;
#[doc = "Field `SE2L` writer - SE2L"]
pub type SE2L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `ELCL` reader - ELCL"]
pub type ELCL_R = crate::BitReader<bool>;
#[doc = "Field `ELCL` writer - ELCL"]
pub type ELCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `IWTL` reader - IWTL"]
pub type IWTL_R = crate::BitReader<bool>;
#[doc = "Field `IWTL` writer - IWTL"]
pub type IWTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `WTL` reader - WTL"]
pub type WTL_R = crate::BitReader<bool>;
#[doc = "Field `WTL` writer - WTL"]
pub type WTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `AWL` reader - AWL"]
pub type AWL_R = crate::BitReader<bool>;
#[doc = "Field `AWL` writer - AWL"]
pub type AWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
#[doc = "Field `CERL` reader - CERL"]
pub type CERL_R = crate::BitReader<bool>;
#[doc = "Field `CERL` writer - CERL"]
pub type CERL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTILS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SBCL"]
    #[inline(always)]
    pub fn sbcl(&self) -> SBCL_R {
        SBCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMCL"]
    #[inline(always)]
    pub fn smcl(&self) -> SMCL_R {
        SMCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSML"]
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOGL"]
    #[inline(always)]
    pub fn sogl(&self) -> SOGL_R {
        SOGL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTMIL"]
    #[inline(always)]
    pub fn rtmil(&self) -> RTMIL_R {
        RTMIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TTMIL"]
    #[inline(always)]
    pub fn ttmil(&self) -> TTMIL_R {
        TTMIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWEL"]
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTWL"]
    #[inline(always)]
    pub fn gtwl(&self) -> GTWL_R {
        GTWL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTDL"]
    #[inline(always)]
    pub fn gtdl(&self) -> GTDL_R {
        GTDL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTEL"]
    #[inline(always)]
    pub fn gtel(&self) -> GTEL_R {
        GTEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUL"]
    #[inline(always)]
    pub fn txul(&self) -> TXUL_R {
        TXUL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXOL"]
    #[inline(always)]
    pub fn txol(&self) -> TXOL_R {
        TXOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SE1L"]
    #[inline(always)]
    pub fn se1l(&self) -> SE1L_R {
        SE1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SE2L"]
    #[inline(always)]
    pub fn se2l(&self) -> SE2L_R {
        SE2L_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ELCL"]
    #[inline(always)]
    pub fn elcl(&self) -> ELCL_R {
        ELCL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IWTL"]
    #[inline(always)]
    pub fn iwtl(&self) -> IWTL_R {
        IWTL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WTL"]
    #[inline(always)]
    pub fn wtl(&self) -> WTL_R {
        WTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWL"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CERL"]
    #[inline(always)]
    pub fn cerl(&self) -> CERL_R {
        CERL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBCL"]
    #[inline(always)]
    #[must_use]
    pub fn sbcl(&mut self) -> SBCL_W<0> {
        SBCL_W::new(self)
    }
    #[doc = "Bit 1 - SMCL"]
    #[inline(always)]
    #[must_use]
    pub fn smcl(&mut self) -> SMCL_W<1> {
        SMCL_W::new(self)
    }
    #[doc = "Bit 2 - CSML"]
    #[inline(always)]
    #[must_use]
    pub fn csml(&mut self) -> CSML_W<2> {
        CSML_W::new(self)
    }
    #[doc = "Bit 3 - SOGL"]
    #[inline(always)]
    #[must_use]
    pub fn sogl(&mut self) -> SOGL_W<3> {
        SOGL_W::new(self)
    }
    #[doc = "Bit 4 - RTMIL"]
    #[inline(always)]
    #[must_use]
    pub fn rtmil(&mut self) -> RTMIL_W<4> {
        RTMIL_W::new(self)
    }
    #[doc = "Bit 5 - TTMIL"]
    #[inline(always)]
    #[must_use]
    pub fn ttmil(&mut self) -> TTMIL_W<5> {
        TTMIL_W::new(self)
    }
    #[doc = "Bit 6 - SWEL"]
    #[inline(always)]
    #[must_use]
    pub fn swel(&mut self) -> SWEL_W<6> {
        SWEL_W::new(self)
    }
    #[doc = "Bit 7 - GTWL"]
    #[inline(always)]
    #[must_use]
    pub fn gtwl(&mut self) -> GTWL_W<7> {
        GTWL_W::new(self)
    }
    #[doc = "Bit 8 - GTDL"]
    #[inline(always)]
    #[must_use]
    pub fn gtdl(&mut self) -> GTDL_W<8> {
        GTDL_W::new(self)
    }
    #[doc = "Bit 9 - GTEL"]
    #[inline(always)]
    #[must_use]
    pub fn gtel(&mut self) -> GTEL_W<9> {
        GTEL_W::new(self)
    }
    #[doc = "Bit 10 - TXUL"]
    #[inline(always)]
    #[must_use]
    pub fn txul(&mut self) -> TXUL_W<10> {
        TXUL_W::new(self)
    }
    #[doc = "Bit 11 - TXOL"]
    #[inline(always)]
    #[must_use]
    pub fn txol(&mut self) -> TXOL_W<11> {
        TXOL_W::new(self)
    }
    #[doc = "Bit 12 - SE1L"]
    #[inline(always)]
    #[must_use]
    pub fn se1l(&mut self) -> SE1L_W<12> {
        SE1L_W::new(self)
    }
    #[doc = "Bit 13 - SE2L"]
    #[inline(always)]
    #[must_use]
    pub fn se2l(&mut self) -> SE2L_W<13> {
        SE2L_W::new(self)
    }
    #[doc = "Bit 14 - ELCL"]
    #[inline(always)]
    #[must_use]
    pub fn elcl(&mut self) -> ELCL_W<14> {
        ELCL_W::new(self)
    }
    #[doc = "Bit 15 - IWTL"]
    #[inline(always)]
    #[must_use]
    pub fn iwtl(&mut self) -> IWTL_W<15> {
        IWTL_W::new(self)
    }
    #[doc = "Bit 16 - WTL"]
    #[inline(always)]
    #[must_use]
    pub fn wtl(&mut self) -> WTL_W<16> {
        WTL_W::new(self)
    }
    #[doc = "Bit 17 - AWL"]
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<17> {
        AWL_W::new(self)
    }
    #[doc = "Bit 18 - CERL"]
    #[inline(always)]
    #[must_use]
    pub fn cerl(&mut self) -> CERL_W<18> {
        CERL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttils](index.html) module"]
pub struct FDCAN_TTILS_SPEC;
impl crate::RegisterSpec for FDCAN_TTILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttils::R](R) reader structure"]
impl crate::Readable for FDCAN_TTILS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttils::W](W) writer structure"]
impl crate::Writable for FDCAN_TTILS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTILS to value 0"]
impl crate::Resettable for FDCAN_TTILS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
