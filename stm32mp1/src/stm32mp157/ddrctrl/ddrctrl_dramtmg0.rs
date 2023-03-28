#[doc = "Register `DDRCTRL_DRAMTMG0` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG0` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG0_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RAS_MIN` reader - T_RAS_MIN"]
pub type T_RAS_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RAS_MIN` writer - T_RAS_MIN"]
pub type T_RAS_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_RAS_MAX` reader - T_RAS_MAX"]
pub type T_RAS_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RAS_MAX` writer - T_RAS_MAX"]
pub type T_RAS_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG0_SPEC, u8, u8, 7, O>;
#[doc = "Field `T_FAW` reader - T_FAW"]
pub type T_FAW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_FAW` writer - T_FAW"]
pub type T_FAW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `WR2PRE` reader - WR2PRE"]
pub type WR2PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR2PRE` writer - WR2PRE"]
pub type WR2PRE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:5 - T_RAS_MIN"]
    #[inline(always)]
    pub fn t_ras_min(&self) -> T_RAS_MIN_R {
        T_RAS_MIN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - T_RAS_MAX"]
    #[inline(always)]
    pub fn t_ras_max(&self) -> T_RAS_MAX_R {
        T_RAS_MAX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - T_FAW"]
    #[inline(always)]
    pub fn t_faw(&self) -> T_FAW_R {
        T_FAW_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:30 - WR2PRE"]
    #[inline(always)]
    pub fn wr2pre(&self) -> WR2PRE_R {
        WR2PRE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - T_RAS_MIN"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras_min(&mut self) -> T_RAS_MIN_W<0> {
        T_RAS_MIN_W::new(self)
    }
    #[doc = "Bits 8:14 - T_RAS_MAX"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras_max(&mut self) -> T_RAS_MAX_W<8> {
        T_RAS_MAX_W::new(self)
    }
    #[doc = "Bits 16:21 - T_FAW"]
    #[inline(always)]
    #[must_use]
    pub fn t_faw(&mut self) -> T_FAW_W<16> {
        T_FAW_W::new(self)
    }
    #[doc = "Bits 24:30 - WR2PRE"]
    #[inline(always)]
    #[must_use]
    pub fn wr2pre(&mut self) -> WR2PRE_W<24> {
        WR2PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg0](index.html) module"]
pub struct DDRCTRL_DRAMTMG0_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG0 to value 0x0f10_1b0f"]
impl crate::Resettable for DDRCTRL_DRAMTMG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f10_1b0f;
}
