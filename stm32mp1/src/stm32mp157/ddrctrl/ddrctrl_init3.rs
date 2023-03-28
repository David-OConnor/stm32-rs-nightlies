#[doc = "Register `DDRCTRL_INIT3` reader"]
pub struct R(crate::R<DDRCTRL_INIT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_INIT3` writer"]
pub struct W(crate::W<DDRCTRL_INIT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT3_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMR` reader - EMR"]
pub type EMR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMR` writer - EMR"]
pub type EMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRCTRL_INIT3_SPEC, u16, u16, 16, O>;
#[doc = "Field `MR` reader - MR"]
pub type MR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MR` writer - MR"]
pub type MR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRCTRL_INIT3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - EMR"]
    #[inline(always)]
    pub fn emr(&self) -> EMR_R {
        EMR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MR"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EMR"]
    #[inline(always)]
    #[must_use]
    pub fn emr(&mut self) -> EMR_W<0> {
        EMR_W::new(self)
    }
    #[doc = "Bits 16:31 - MR"]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<16> {
        MR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM initialization register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init3](index.html) module"]
pub struct DDRCTRL_INIT3_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_init3::R](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init3::W](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT3 to value 0x0510"]
impl crate::Resettable for DDRCTRL_INIT3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0510;
}
