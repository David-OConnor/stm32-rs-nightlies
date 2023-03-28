#[doc = "Register `DDRCTRL_DFIPHYMSTR` reader"]
pub struct R(crate::R<DDRCTRL_DFIPHYMSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFIPHYMSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFIPHYMSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFIPHYMSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFIPHYMSTR` writer"]
pub struct W(crate::W<DDRCTRL_DFIPHYMSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFIPHYMSTR_SPEC>;
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
impl From<crate::W<DDRCTRL_DFIPHYMSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFIPHYMSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_PHYMSTR_EN` reader - DFI_PHYMSTR_EN"]
pub type DFI_PHYMSTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `DFI_PHYMSTR_EN` writer - DFI_PHYMSTR_EN"]
pub type DFI_PHYMSTR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DFIPHYMSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DFI_PHYMSTR_EN"]
    #[inline(always)]
    pub fn dfi_phymstr_en(&self) -> DFI_PHYMSTR_EN_R {
        DFI_PHYMSTR_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_PHYMSTR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_phymstr_en(&mut self) -> DFI_PHYMSTR_EN_W<0> {
        DFI_PHYMSTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DFI PHY master register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfiphymstr](index.html) module"]
pub struct DDRCTRL_DFIPHYMSTR_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFIPHYMSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfiphymstr::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIPHYMSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfiphymstr::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIPHYMSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFIPHYMSTR to value 0x01"]
impl crate::Resettable for DDRCTRL_DFIPHYMSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
