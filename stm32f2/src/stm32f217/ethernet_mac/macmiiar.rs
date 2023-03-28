#[doc = "Register `MACMIIAR` reader"]
pub struct R(crate::R<MACMIIAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMIIAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMIIAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMIIAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACMIIAR` writer"]
pub struct W(crate::W<MACMIIAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMIIAR_SPEC>;
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
impl From<crate::W<MACMIIAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMIIAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB` reader - MII busy"]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - MII busy"]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMIIAR_SPEC, bool, O>;
#[doc = "Field `MW` reader - MII write"]
pub type MW_R = crate::BitReader<bool>;
#[doc = "Field `MW` writer - MII write"]
pub type MW_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMIIAR_SPEC, bool, O>;
#[doc = "Field `CR` reader - Clock range"]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - Clock range"]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MR` reader - MII register"]
pub type MR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MR` writer - MII register"]
pub type MR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA` reader - PHY address"]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - PHY address"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIAR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<0> {
        MB_W::new(self)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MW_W<1> {
        MW_W::new(self)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<2> {
        CR_W::new(self)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<6> {
        MR_W::new(self)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<11> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MII address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmiiar](index.html) module"]
pub struct MACMIIAR_SPEC;
impl crate::RegisterSpec for MACMIIAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macmiiar::R](R) reader structure"]
impl crate::Readable for MACMIIAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macmiiar::W](W) writer structure"]
impl crate::Writable for MACMIIAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACMIIAR to value 0"]
impl crate::Resettable for MACMIIAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
