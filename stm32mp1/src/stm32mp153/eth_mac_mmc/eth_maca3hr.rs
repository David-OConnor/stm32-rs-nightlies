#[doc = "Register `ETH_MACA3HR` reader"]
pub struct R(crate::R<ETH_MACA3HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACA3HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACA3HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACA3HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACA3HR` writer"]
pub struct W(crate::W<ETH_MACA3HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACA3HR_SPEC>;
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
impl From<crate::W<ETH_MACA3HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACA3HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRHI` reader - ADDRHI"]
pub type ADDRHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRHI` writer - ADDRHI"]
pub type ADDRHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACA3HR_SPEC, u16, u16, 16, O>;
#[doc = "Field `MBC` reader - MBC"]
pub type MBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBC` writer - MBC"]
pub type MBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACA3HR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SA` reader - SA"]
pub type SA_R = crate::BitReader<bool>;
#[doc = "Field `SA` writer - SA"]
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACA3HR_SPEC, bool, O>;
#[doc = "Field `AE` reader - AE"]
pub type AE_R = crate::BitReader<bool>;
#[doc = "Field `AE` writer - AE"]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACA3HR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<0> {
        ADDRHI_W::new(self)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<24> {
        MBC_W::new(self)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<30> {
        SA_W::new(self)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca3hr](index.html) module"]
pub struct ETH_MACA3HR_SPEC;
impl crate::RegisterSpec for ETH_MACA3HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maca3hr::R](R) reader structure"]
impl crate::Readable for ETH_MACA3HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maca3hr::W](W) writer structure"]
impl crate::Writable for ETH_MACA3HR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACA3HR to value 0xffff"]
impl crate::Resettable for ETH_MACA3HR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
