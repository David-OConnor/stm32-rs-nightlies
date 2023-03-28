#[doc = "Register `ETH_MACL3L4C1R` reader"]
pub struct R(crate::R<ETH_MACL3L4C1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL3L4C1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL3L4C1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL3L4C1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACL3L4C1R` writer"]
pub struct W(crate::W<ETH_MACL3L4C1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL3L4C1R_SPEC>;
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
impl From<crate::W<ETH_MACL3L4C1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL3L4C1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3PEN1` reader - L3PEN1"]
pub type L3PEN1_R = crate::BitReader<bool>;
#[doc = "Field `L3PEN1` writer - L3PEN1"]
pub type L3PEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L3SAM1` reader - L3SAM1"]
pub type L3SAM1_R = crate::BitReader<bool>;
#[doc = "Field `L3SAM1` writer - L3SAM1"]
pub type L3SAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L3SAIM1` reader - L3SAIM1"]
pub type L3SAIM1_R = crate::BitReader<bool>;
#[doc = "Field `L3SAIM1` writer - L3SAIM1"]
pub type L3SAIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L3DAM1` reader - L3DAM1"]
pub type L3DAM1_R = crate::BitReader<bool>;
#[doc = "Field `L3DAM1` writer - L3DAM1"]
pub type L3DAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L3DAIM1` reader - L3DAIM1"]
pub type L3DAIM1_R = crate::BitReader<bool>;
#[doc = "Field `L3DAIM1` writer - L3DAIM1"]
pub type L3DAIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L3HSBM1` reader - L3HSBM1"]
pub type L3HSBM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L3HSBM1` writer - L3HSBM1"]
pub type L3HSBM1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACL3L4C1R_SPEC, u8, u8, 5, O>;
#[doc = "Field `L3HDBM1` reader - L3HDBM1"]
pub type L3HDBM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L3HDBM1` writer - L3HDBM1"]
pub type L3HDBM1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACL3L4C1R_SPEC, u8, u8, 5, O>;
#[doc = "Field `L4PEN1` reader - L4PEN1"]
pub type L4PEN1_R = crate::BitReader<bool>;
#[doc = "Field `L4PEN1` writer - L4PEN1"]
pub type L4PEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L4SPM1` reader - L4SPM1"]
pub type L4SPM1_R = crate::BitReader<bool>;
#[doc = "Field `L4SPM1` writer - L4SPM1"]
pub type L4SPM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L4SPIM1` reader - L4SPIM1"]
pub type L4SPIM1_R = crate::BitReader<bool>;
#[doc = "Field `L4SPIM1` writer - L4SPIM1"]
pub type L4SPIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L4DPM1` reader - L4DPM1"]
pub type L4DPM1_R = crate::BitReader<bool>;
#[doc = "Field `L4DPM1` writer - L4DPM1"]
pub type L4DPM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
#[doc = "Field `L4DPIM1` reader - L4DPIM1"]
pub type L4DPIM1_R = crate::BitReader<bool>;
#[doc = "Field `L4DPIM1` writer - L4DPIM1"]
pub type L4DPIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACL3L4C1R_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    pub fn l3pen1(&self) -> L3PEN1_R {
        L3PEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    pub fn l3sam1(&self) -> L3SAM1_R {
        L3SAM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    pub fn l3saim1(&self) -> L3SAIM1_R {
        L3SAIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    pub fn l3dam1(&self) -> L3DAM1_R {
        L3DAM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    pub fn l3daim1(&self) -> L3DAIM1_R {
        L3DAIM1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    pub fn l3hsbm1(&self) -> L3HSBM1_R {
        L3HSBM1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    pub fn l3hdbm1(&self) -> L3HDBM1_R {
        L3HDBM1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    pub fn l4pen1(&self) -> L4PEN1_R {
        L4PEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    pub fn l4spm1(&self) -> L4SPM1_R {
        L4SPM1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    pub fn l4spim1(&self) -> L4SPIM1_R {
        L4SPIM1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    pub fn l4dpm1(&self) -> L4DPM1_R {
        L4DPM1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    pub fn l4dpim1(&self) -> L4DPIM1_R {
        L4DPIM1_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    #[must_use]
    pub fn l3pen1(&mut self) -> L3PEN1_W<0> {
        L3PEN1_W::new(self)
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3sam1(&mut self) -> L3SAM1_W<2> {
        L3SAM1_W::new(self)
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3saim1(&mut self) -> L3SAIM1_W<3> {
        L3SAIM1_W::new(self)
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3dam1(&mut self) -> L3DAM1_W<4> {
        L3DAM1_W::new(self)
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3daim1(&mut self) -> L3DAIM1_W<5> {
        L3DAIM1_W::new(self)
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3hsbm1(&mut self) -> L3HSBM1_W<6> {
        L3HSBM1_W::new(self)
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3hdbm1(&mut self) -> L3HDBM1_W<11> {
        L3HDBM1_W::new(self)
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    #[must_use]
    pub fn l4pen1(&mut self) -> L4PEN1_W<16> {
        L4PEN1_W::new(self)
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4spm1(&mut self) -> L4SPM1_W<18> {
        L4SPM1_W::new(self)
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4spim1(&mut self) -> L4SPIM1_W<19> {
        L4SPIM1_W::new(self)
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4dpm1(&mut self) -> L4DPM1_W<20> {
        L4DPM1_W::new(self)
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4dpim1(&mut self) -> L4DPIM1_W<21> {
        L4DPIM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3l4c1r](index.html) module"]
pub struct ETH_MACL3L4C1R_SPEC;
impl crate::RegisterSpec for ETH_MACL3L4C1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macl3l4c1r::R](R) reader structure"]
impl crate::Readable for ETH_MACL3L4C1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macl3l4c1r::W](W) writer structure"]
impl crate::Writable for ETH_MACL3L4C1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACL3L4C1R to value 0"]
impl crate::Resettable for ETH_MACL3L4C1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
