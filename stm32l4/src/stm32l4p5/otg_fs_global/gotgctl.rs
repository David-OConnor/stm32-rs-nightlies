#[doc = "Register `GOTGCTL` reader"]
pub struct R(crate::R<GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGCTL` writer"]
pub struct W(crate::W<GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGCTL_SPEC>;
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
impl From<crate::W<GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRQSCS` reader - Session request success"]
pub type SRQSCS_R = crate::BitReader<bool>;
#[doc = "Field `SRQ` reader - Session request"]
pub type SRQ_R = crate::BitReader<bool>;
#[doc = "Field `SRQ` writer - Session request"]
pub type SRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `VBVALOEN` reader - VBUS valid override enable"]
pub type VBVALOEN_R = crate::BitReader<bool>;
#[doc = "Field `VBVALOEN` writer - VBUS valid override enable"]
pub type VBVALOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `VBVALOVA` reader - VBUS valid override value"]
pub type VBVALOVA_R = crate::BitReader<bool>;
#[doc = "Field `VBVALOVA` writer - VBUS valid override value"]
pub type VBVALOVA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `AVALOEN` reader - A-peripheral session valid override enable"]
pub type AVALOEN_R = crate::BitReader<bool>;
#[doc = "Field `AVALOEN` writer - A-peripheral session valid override enable"]
pub type AVALOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `AVALOVAL` reader - A-peripheral session valid override value"]
pub type AVALOVAL_R = crate::BitReader<bool>;
#[doc = "Field `AVALOVAL` writer - A-peripheral session valid override value"]
pub type AVALOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `BVALOEN` reader - B-peripheral session valid override enable"]
pub type BVALOEN_R = crate::BitReader<bool>;
#[doc = "Field `BVALOEN` writer - B-peripheral session valid override enable"]
pub type BVALOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `BVALOVAL` reader - B-peripheral session valid override value"]
pub type BVALOVAL_R = crate::BitReader<bool>;
#[doc = "Field `BVALOVAL` writer - B-peripheral session valid override value"]
pub type BVALOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `HNGSCS` reader - Host negotiation success"]
pub type HNGSCS_R = crate::BitReader<bool>;
#[doc = "Field `HNPRQ` reader - HNP request"]
pub type HNPRQ_R = crate::BitReader<bool>;
#[doc = "Field `HNPRQ` writer - HNP request"]
pub type HNPRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `HSHNPEN` reader - Host set HNP enable"]
pub type HSHNPEN_R = crate::BitReader<bool>;
#[doc = "Field `HSHNPEN` writer - Host set HNP enable"]
pub type HSHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DHNPEN_R = crate::BitReader<bool>;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `EHEN` reader - Embedded host enable"]
pub type EHEN_R = crate::BitReader<bool>;
#[doc = "Field `EHEN` writer - Embedded host enable"]
pub type EHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `CIDSTS` reader - Connector ID status"]
pub type CIDSTS_R = crate::BitReader<bool>;
#[doc = "Field `DBCT` reader - Long/short debounce time"]
pub type DBCT_R = crate::BitReader<bool>;
#[doc = "Field `ASVLD` reader - A-session valid"]
pub type ASVLD_R = crate::BitReader<bool>;
#[doc = "Field `BSVLD` reader - B-session valid"]
pub type BSVLD_R = crate::BitReader<bool>;
#[doc = "Field `OTGVER` reader - OTG version"]
pub type OTGVER_R = crate::BitReader<bool>;
#[doc = "Field `OTGVER` writer - OTG version"]
pub type OTGVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `CURMOD` reader - Current mode of operation"]
pub type CURMOD_R = crate::BitReader<bool>;
#[doc = "Field `CURMOD` writer - Current mode of operation"]
pub type CURMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    pub fn vbvalova(&self) -> VBVALOVA_R {
        VBVALOVA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Current mode of operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    #[must_use]
    pub fn srq(&mut self) -> SRQ_W<1> {
        SRQ_W::new(self)
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W<2> {
        VBVALOEN_W::new(self)
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalova(&mut self) -> VBVALOVA_W<3> {
        VBVALOVA_W::new(self)
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    #[must_use]
    pub fn avaloen(&mut self) -> AVALOEN_W<4> {
        AVALOEN_W::new(self)
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    #[must_use]
    pub fn avaloval(&mut self) -> AVALOVAL_W<5> {
        AVALOVAL_W::new(self)
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    #[must_use]
    pub fn bvaloen(&mut self) -> BVALOEN_W<6> {
        BVALOEN_W::new(self)
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    #[must_use]
    pub fn bvaloval(&mut self) -> BVALOVAL_W<7> {
        BVALOVAL_W::new(self)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    #[must_use]
    pub fn hnprq(&mut self) -> HNPRQ_W<9> {
        HNPRQ_W::new(self)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    #[must_use]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<10> {
        HSHNPEN_W::new(self)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dhnpen(&mut self) -> DHNPEN_W<11> {
        DHNPEN_W::new(self)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    #[must_use]
    pub fn ehen(&mut self) -> EHEN_W<12> {
        EHEN_W::new(self)
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<20> {
        OTGVER_W::new(self)
    }
    #[doc = "Bit 21 - Current mode of operation"]
    #[inline(always)]
    #[must_use]
    pub fn curmod(&mut self) -> CURMOD_W<21> {
        CURMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](index.html) module"]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgctl::R](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0800"]
impl crate::Resettable for GOTGCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
