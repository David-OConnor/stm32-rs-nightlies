#[doc = "Register `DDRCTRL_DRAMTMG7` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG7` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG7_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_CKPDX` reader - T_CKPDX"]
pub type T_CKPDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKPDX` writer - T_CKPDX"]
pub type T_CKPDX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG7_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_CKPDE` reader - T_CKPDE"]
pub type T_CKPDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKPDE` writer - T_CKPDE"]
pub type T_CKPDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG7_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    pub fn t_ckpdx(&self) -> T_CKPDX_R {
        T_CKPDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    pub fn t_ckpde(&self) -> T_CKPDE_R {
        T_CKPDE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckpdx(&mut self) -> T_CKPDX_W<0> {
        T_CKPDX_W::new(self)
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckpde(&mut self) -> T_CKPDE_W<8> {
        T_CKPDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg7](index.html) module"]
pub struct DDRCTRL_DRAMTMG7_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg7::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg7::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG7 to value 0x0202"]
impl crate::Resettable for DDRCTRL_DRAMTMG7_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202;
}
