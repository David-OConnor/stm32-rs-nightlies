#[doc = "Register `DDRCTRL_DRAMTMG1` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG1` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG1_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RC` reader - T_RC"]
pub type T_RC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RC` writer - T_RC"]
pub type T_RC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG1_SPEC, u8, u8, 7, O>;
#[doc = "Field `RD2PRE` reader - RD2PRE"]
pub type RD2PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD2PRE` writer - RD2PRE"]
pub type RD2PRE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG1_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_XP` reader - T_XP"]
pub type T_XP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_XP` writer - T_XP"]
pub type T_XP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:6 - T_RC"]
    #[inline(always)]
    pub fn t_rc(&self) -> T_RC_R {
        T_RC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - RD2PRE"]
    #[inline(always)]
    pub fn rd2pre(&self) -> RD2PRE_R {
        RD2PRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - T_XP"]
    #[inline(always)]
    pub fn t_xp(&self) -> T_XP_R {
        T_XP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - T_RC"]
    #[inline(always)]
    #[must_use]
    pub fn t_rc(&mut self) -> T_RC_W<0> {
        T_RC_W::new(self)
    }
    #[doc = "Bits 8:13 - RD2PRE"]
    #[inline(always)]
    #[must_use]
    pub fn rd2pre(&mut self) -> RD2PRE_W<8> {
        RD2PRE_W::new(self)
    }
    #[doc = "Bits 16:20 - T_XP"]
    #[inline(always)]
    #[must_use]
    pub fn t_xp(&mut self) -> T_XP_W<16> {
        T_XP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg1](index.html) module"]
pub struct DDRCTRL_DRAMTMG1_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG1 to value 0x0008_0414"]
impl crate::Resettable for DDRCTRL_DRAMTMG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0414;
}
