#[doc = "Register `DDRCTRL_DRAMTMG3` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG3` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG3_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_MOD` reader - T_MOD"]
pub type T_MOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_MOD` writer - T_MOD"]
pub type T_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG3_SPEC, u16, u16, 10, O>;
#[doc = "Field `T_MRD` reader - T_MRD"]
pub type T_MRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MRD` writer - T_MRD"]
pub type T_MRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG3_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_MRW` reader - T_MRW"]
pub type T_MRW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_MRW` writer - T_MRW"]
pub type T_MRW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG3_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    pub fn t_mod(&self) -> T_MOD_R {
        T_MOD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    pub fn t_mrd(&self) -> T_MRD_R {
        T_MRD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    pub fn t_mrw(&self) -> T_MRW_R {
        T_MRW_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    #[must_use]
    pub fn t_mod(&mut self) -> T_MOD_W<0> {
        T_MOD_W::new(self)
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrd(&mut self) -> T_MRD_W<12> {
        T_MRD_W::new(self)
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrw(&mut self) -> T_MRW_W<20> {
        T_MRW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg3](index.html) module"]
pub struct DDRCTRL_DRAMTMG3_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg3::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg3::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG3 to value 0x0050_400c"]
impl crate::Resettable for DDRCTRL_DRAMTMG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0050_400c;
}
