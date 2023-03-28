#[doc = "Register `RXGFC` reader"]
pub struct R(crate::R<RXGFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXGFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXGFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXGFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXGFC` writer"]
pub struct W(crate::W<RXGFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXGFC_SPEC>;
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
impl From<crate::W<RXGFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXGFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRFE` reader - RRFE"]
pub type RRFE_R = crate::BitReader<bool>;
#[doc = "Field `RRFE` writer - RRFE"]
pub type RRFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXGFC_SPEC, bool, O>;
#[doc = "Field `RRFS` reader - RRFS"]
pub type RRFS_R = crate::BitReader<bool>;
#[doc = "Field `RRFS` writer - RRFS"]
pub type RRFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXGFC_SPEC, bool, O>;
#[doc = "Field `ANFE` writer - ANFE"]
pub type ANFE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXGFC_SPEC, u8, u8, 2, O>;
#[doc = "Field `ANFS` writer - ANFS"]
pub type ANFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXGFC_SPEC, u8, u8, 2, O>;
#[doc = "Field `F1OM` reader - FIFO 1 operation mode"]
pub type F1OM_R = crate::BitReader<bool>;
#[doc = "Field `F1OM` writer - FIFO 1 operation mode"]
pub type F1OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXGFC_SPEC, bool, O>;
#[doc = "Field `F0OM` reader - FIFO 0 operation mode"]
pub type F0OM_R = crate::BitReader<bool>;
#[doc = "Field `F0OM` writer - FIFO 0 operation mode"]
pub type F0OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXGFC_SPEC, bool, O>;
#[doc = "Field `LSS` reader - List size standard"]
pub type LSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSS` writer - List size standard"]
pub type LSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXGFC_SPEC, u8, u8, 5, O>;
#[doc = "Field `LSE` reader - List size extended"]
pub type LSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSE` writer - List size extended"]
pub type LSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXGFC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - List size standard"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - List size extended"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<0> {
        RRFE_W::new(self)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<1> {
        RRFS_W::new(self)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<2> {
        ANFE_W::new(self)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<4> {
        ANFS_W::new(self)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1OM_W<8> {
        F1OM_W::new(self)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<9> {
        F0OM_W::new(self)
    }
    #[doc = "Bits 16:20 - List size standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<16> {
        LSS_W::new(self)
    }
    #[doc = "Bits 24:27 - List size extended"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<24> {
        LSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxgfc](index.html) module"]
pub struct RXGFC_SPEC;
impl crate::RegisterSpec for RXGFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxgfc::R](R) reader structure"]
impl crate::Readable for RXGFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxgfc::W](W) writer structure"]
impl crate::Writable for RXGFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXGFC to value 0"]
impl crate::Resettable for RXGFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
