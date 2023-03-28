#[doc = "Register `QMEM1%s` reader"]
pub struct R(crate::R<QMEM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QMEM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QMEM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QMEM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QMEM1%s` writer"]
pub struct W(crate::W<QMEM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QMEM1_SPEC>;
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
impl From<crate::W<QMEM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QMEM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QMem_RAM` reader - QMem RAM"]
pub type QMEM_RAM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QMem_RAM` writer - QMem RAM"]
pub type QMEM_RAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QMEM1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    pub fn qmem_ram(&self) -> QMEM_RAM_R {
        QMEM_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    #[must_use]
    pub fn qmem_ram(&mut self) -> QMEM_RAM_W<0> {
        QMEM_RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1](index.html) module"]
pub struct QMEM1_SPEC;
impl crate::RegisterSpec for QMEM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qmem1::R](R) reader structure"]
impl crate::Readable for QMEM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qmem1::W](W) writer structure"]
impl crate::Writable for QMEM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QMEM1%s to value 0"]
impl crate::Resettable for QMEM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
